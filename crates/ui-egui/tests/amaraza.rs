//! The Amaraza PDF fork in the real shell (egui_kittest; docs/amaraza/adr/0001-puntos-de-enganche.md
//! §5.2): the "hola Amaraza" command from the Amaraza menu and the palette, and the fork's Action
//! Wizard step in the step picker and in saved actions.

use egui::Key;
use egui::accesskit::Role;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_engine::actions::{Action, Step};
use pdfcraft_engine::amaraza;
use pdfcraft_ui_egui::PdfCraftApp;

/// A synthetic PDF with `n` empty pages.
fn fixture(n: usize) -> Vec<u8> {
    let mut objs: Vec<String> = vec!["<< /Type /Catalog /Pages 2 0 R >>".into()];
    let kids: Vec<String> = (0..n).map(|i| format!("{} 0 R", 3 + i)).collect();
    objs.push(format!("<< /Type /Pages /Kids [{}] /Count {n} /MediaBox [0 0 200 300] >>", kids.join(" ")));
    for _ in 0..n {
        objs.push("<< /Type /Page /Parent 2 0 R >>".into());
    }
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (i, o) in objs.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n{o}\nendobj\n", i + 1).as_bytes());
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objs.len() + 1).as_bytes());
    for o in offsets {
        out.extend_from_slice(format!("{o:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objs.len() + 1).as_bytes());
    out
}

fn harness() -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(|_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.open_bytes("doc.pdf", None, fixture(3)).unwrap();
        app
    });
    h.run_steps(4);
    h
}

fn toast(h: &Harness<'_, PdfCraftApp>) -> String {
    h.state().toast.as_ref().map(|t| t.0.clone()).unwrap_or_default()
}

fn greeting(pages: usize) -> String {
    format!("Hola, Amaraza PDF {} · {pages} páginas", env!("CARGO_PKG_VERSION"))
}

#[test]
fn the_hello_command_greets_with_the_page_count() {
    let mut h = harness();
    assert!(h.state_mut().execute("amaraza.hello"));
    h.run_steps(2);
    assert_eq!(toast(&h), greeting(3));
}

#[test]
fn the_amaraza_menu_runs_the_command() {
    let mut h = harness();
    h.get_by_label("Menu").click();
    h.run_steps(2);
    h.get_by_label("Amaraza ⏵").hover();
    h.run_steps(3);
    // A menu item's label also carries its shortcut text, as in the upstream's menu tests.
    h.get_by_label_contains("Hola, Amaraza").click();
    h.run_steps(3);
    assert_eq!(toast(&h), greeting(3));
}

#[test]
fn the_palette_finds_and_runs_the_command() {
    let mut h = harness();
    h.state_mut().set_option("palette", "hola, amaraza").unwrap();
    h.run_steps(3);
    h.get_by_label("Hola, Amaraza");
    h.key_press(Key::Enter);
    h.run_steps(3);
    assert!(!h.state().palette_open);
    assert_eq!(toast(&h), greeting(3));
}

#[test]
fn the_step_picker_offers_the_fork_step() {
    let mut h = harness();
    assert!(h.state_mut().execute("actions.wizard"));
    h.run_steps(2);
    h.get_by_label("New Action…").click();
    h.run_steps(2);
    // The step picker is a combo box: it shows its choice (the first step) as its value.
    h.query_all_by(|n| n.role() == Role::ComboBox && n.value().is_some_and(|v| v == "Recognize text")).next().expect("the step picker").click();
    h.run_steps(2);
    // Pick the option by its AccessKit click action, as the control channel's ui.click does.
    h.get_by_label("Saludo de Amaraza").click_accesskit();
    h.run_steps(2);
    h.get_by_label("Add Step").click();
    h.run_steps(2);
    let (_, a) = h.state().wizard.editing.as_ref().unwrap();
    assert_eq!(a.steps, vec![Step::Amaraza(amaraza::Step::Hello)]);
    // A text label exposes its text as its AccessKit value.
    let listed = h.query_all_by(|n| n.role() == Role::Label && n.value().is_some_and(|v| v == "1. Saludo de Amaraza")).next();
    assert!(listed.is_some(), "the action's step list shows the fork step");
}

#[test]
fn saved_actions_keep_the_fork_step() {
    let mut app = PdfCraftApp::new();
    app.set_option("language", "en").unwrap();
    let steps = vec![Step::AddWatermark("BORRADOR".into()), Step::Amaraza(amaraza::Step::Hello)];
    app.custom_actions.push(Action { name: "Saludar".into(), description: String::new(), steps, builtin: false });
    let saved = app.persist();
    let mut fresh = PdfCraftApp::new();
    fresh.restore(&saved);
    assert_eq!(fresh.custom_actions, app.custom_actions);
}

/// Explicit exceptions for string literals containing "PdfCraft" that are not user-facing UI messages.
const ALLOWLIST_EXCEPTIONS: &[(&str, &str)] =
    &[("the running PdfCraft didn't confirm", "Internal io::Error message in single_instance.rs; not shown in UI")];

fn is_exception(s: &str) -> bool {
    ALLOWLIST_EXCEPTIONS.iter().any(|(exc, _)| *exc == s)
}

fn extract_ui_strings(text: &str, out: &mut std::collections::HashSet<String>) {
    // Exclude #[cfg(test)] modules at the bottom of files
    let content = if let Some(idx) = text.find("#[cfg(test)]\nmod ") {
        &text[..idx]
    } else if let Some(idx) = text.find("#[cfg(test)]\r\nmod ") {
        &text[..idx]
    } else {
        text
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
            continue;
        }
        if trimmed.contains("log::") || trimmed.contains("tracing::") {
            continue;
        }
        let mut rest = line;
        while let Some(start) = rest.find('"') {
            let after_open = &rest[start + 1..];
            let mut end = None;
            let mut escaped = false;
            for (i, b) in after_open.bytes().enumerate() {
                if escaped {
                    escaped = false;
                } else if b == b'\\' {
                    escaped = true;
                } else if b == b'"' {
                    end = Some(i);
                    break;
                }
            }
            if let Some(close) = end {
                let literal = &after_open[..close];
                if literal.contains("PdfCraft") && !is_exception(literal) {
                    out.insert(literal.to_string());
                }
                rest = &after_open[close + 1..];
            } else {
                break;
            }
        }
    }
}

fn walk_dir(dir: &std::path::Path, out: &mut std::collections::HashSet<String>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|n| n != "amaraza") {
                    walk_dir(&path, out);
                }
            } else if path.extension().is_some_and(|e| e == "rs") {
                if let Ok(text) = std::fs::read_to_string(&path) {
                    extract_ui_strings(&text, out);
                }
            }
        }
    }
}

#[test]
fn ui_code_scan_has_no_unhandled_pdfcraft() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let ui_src = manifest_dir.join("src");
    let app_src = manifest_dir.join("../../apps/pdfcraft/src");
    let mut keys = std::collections::HashSet::new();
    walk_dir(&ui_src, &mut keys);
    walk_dir(&app_src, &mut keys);
    assert!(!keys.is_empty(), "expected to find UI strings with 'PdfCraft'");
    for key in &keys {
        for lang in pdfcraft_ui_egui::i18n::Lang::all() {
            let translated = pdfcraft_ui_egui::i18n::tr(lang, key);
            assert!(!translated.contains("PdfCraft"), "Language {:?} for key {:?} still contains 'PdfCraft': {:?}", lang.code(), key, translated);
        }
    }
}
