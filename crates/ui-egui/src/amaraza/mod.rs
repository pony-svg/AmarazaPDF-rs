//! Amaraza PDF fork surfaces in the UI (docs/amaraza/adr/0001-puntos-de-enganche.md): running
//! the fork's commands, the Amaraza menu, and the fork's translation layer.
//!
//! A7: this is glue inside an upstream crate, so clippy applies the root's complexity threshold
//! here, not 15 (it has no per-module threshold). It only dispatches; the logic is in
//! `amaraza-ext`.

#![warn(clippy::cognitive_complexity)]

use std::borrow::Cow;

use pdfcraft_engine::amaraza::{COMMAND_PREFIX, MENU};

use crate::PdfCraftApp;

/// The fork's catalog rows for every language: `language<TAB>context<TAB>source<TAB>translation`.
const ROWS: &str = include_str!("i18n.tsv");

/// Whether `id` is one of the fork's commands.
pub(crate) fn handles(id: &str) -> bool {
    id.starts_with(COMMAND_PREFIX)
}

/// Run the fork command `id`; its `Needs` were already checked. `false` for an unknown id, as in
/// `execute`.
pub(crate) fn execute(app: &mut PdfCraftApp, id: &str) -> bool {
    match id {
        "amaraza.hello" => hello(app),
        _ => return false,
    }
    true
}

fn hello(app: &mut PdfCraftApp) {
    let doc = app.active_ids().map(|(_, id)| id);
    match amaraza_ext::hello::greeting(&app.session, doc) {
        Ok(g) => app.notify(g.text()),
        Err(e) => app.notify(e.to_string()),
    }
}

/// The fork's top-level menu, drawn before Help in the main menu.
pub(crate) fn main_menu(app: &mut PdfCraftApp, ui: &mut egui::Ui) {
    ui.menu_button(tl!(MENU), |ui| crate::commands::registry_menu(app, ui, MENU));
}

/// Discovers all unique English `source` strings across all bundled catalogs that contain `"PdfCraft"`.
fn all_pdfcraft_keys() -> &'static [&'static str] {
    static KEYS: std::sync::OnceLock<Vec<&'static str>> = std::sync::OnceLock::new();
    KEYS.get_or_init(|| {
        let mut set = std::collections::HashSet::new();
        for lang in &crate::i18n::LANGUAGES {
            for line in lang.source.lines() {
                if line.trim().is_empty() || line.starts_with('#') {
                    continue;
                }
                let mut cols = line.split('\t');
                if let (Some(_ctx), Some(source), Some(_trans)) = (cols.next(), cols.next(), cols.next())
                    && source.contains("PdfCraft")
                {
                    set.insert(source);
                }
            }
        }
        let mut list: Vec<&'static str> = set.into_iter().collect();
        list.sort_unstable();
        list
    })
}

fn append_fallback_row(out: &mut String, key: &str) {
    let replacement = key.replace("PdfCraft", amaraza_identity::PRODUCT_NAME);
    out.push('\t');
    out.push_str(key);
    out.push('\t');
    out.push_str(&replacement);
    out.push('\n');
}

fn english_catalog() -> String {
    let mut out = String::new();
    for key in all_pdfcraft_keys() {
        append_fallback_row(&mut out, key);
    }
    out
}

fn transform_catalog(source: &str) -> String {
    let mut out = String::with_capacity(source.len() + 1024);
    let mut seen = std::collections::HashSet::new();
    for line in source.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            out.push_str(line);
            out.push('\n');
            continue;
        }
        let mut cols = line.split('\t');
        let (Some(context), Some(src), Some(trans)) = (cols.next(), cols.next(), cols.next()) else {
            out.push_str(line);
            out.push('\n');
            continue;
        };
        seen.insert(src);
        out.push_str(context);
        out.push('\t');
        out.push_str(src);
        out.push('\t');
        if trans.contains("PdfCraft") {
            out.push_str(&trans.replace("PdfCraft", amaraza_identity::PRODUCT_NAME));
        } else {
            out.push_str(trans);
        }
        out.push('\n');
    }
    for key in all_pdfcraft_keys() {
        if !seen.contains(key) {
            append_fallback_row(&mut out, key);
        }
    }
    out
}

/// The catalog text for language `code`: the upstream's `source` transformed in memory
/// (substituting "PdfCraft" with "Amaraza PDF" and injecting English fallbacks for missing
/// keys), plus the fork's rows for that language without their first column, so they go
/// through the same parser and checks.
pub(crate) fn catalog_source(code: &str, source: &'static str) -> Cow<'static, str> {
    let fork_rows: String = ROWS.lines().filter_map(|line| line.strip_prefix(code)?.strip_prefix('\t')).flat_map(|row| [row, "\n"]).collect();
    let base = if code == "en" { english_catalog() } else { transform_catalog(source) };
    if fork_rows.is_empty() { Cow::Owned(base) } else { Cow::Owned(format!("{base}{fork_rows}")) }
}

#[cfg(test)]
mod tests {
    use crate::i18n::{Lang, has};

    /// Every fork row loads in its language: a row the parser rejected would be missing.
    #[test]
    fn every_fork_row_loads_in_its_language() {
        let mut rows = 0;
        for line in super::ROWS.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
            let cols: Vec<&str> = line.split('\t').collect();
            assert_eq!(cols.len(), 4, "language, context, source, translation: {line:?}");
            let lang = Lang::from_code(cols[0]).unwrap_or_else(|| panic!("unknown language {:?}", cols[0]));
            assert!(cols[1].is_empty(), "plain strings only: {line:?}");
            assert!(has(lang, cols[2]), "{}: {:?} did not load", cols[0], cols[2]);
            rows += 1;
        }
        assert!(rows > 0);
    }

    /// Every language but English has every fork text: command labels, the menu and step labels.
    #[test]
    fn every_language_has_every_fork_text() {
        let mut texts: Vec<&str> = pdfcraft_engine::amaraza::COMMANDS.iter().map(|c| c.label).collect();
        texts.push(pdfcraft_engine::amaraza::MENU);
        texts.extend(pdfcraft_engine::amaraza::Step::all().iter().map(|s| s.label()));
        for lang in Lang::all().filter(|l| l.code() != "en") {
            for text in &texts {
                assert!(has(lang, text), "{}: missing {text:?}", lang.code());
            }
        }
    }

    /// The upstream catalog still loads in full with the fork's rows after it.
    #[test]
    fn the_upstream_catalog_still_loads() {
        let ja = Lang::from_code("ja").unwrap();
        assert!(has(ja, "File") && has(ja, "Help"));
        let en_src = super::catalog_source("en", "");
        assert!(en_src.contains(amaraza_identity::PRODUCT_NAME));
        assert!(!en_src.contains("PdfCraft\tPdfCraft"));
    }
}
