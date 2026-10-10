//! Project links: the Help menu and the About dialog open Amaraza PDF's repository and credit
//! PdfCraft in plain text. Nothing links to the ArtCraft site or its community: the ArtCraft brand
//! licence leaves forks a plain-text credit only (docs/amaraza/PLAN.md, AR0.1).

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_engine::links;
use pdfcraft_ui_egui::{Dialog, PdfCraftApp};

fn harness(setup: impl FnOnce(&mut PdfCraftApp) + 'static) -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        setup(&mut app);
        app
    });
    h.run_steps(4);
    h
}

const ONE_PAGE: &[u8] = b"%PDF-1.7\n1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj\n3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] >> endobj\ntrailer << /Root 1 0 R >>\n%%EOF";

#[test]
fn no_artcraft_community_on_the_home_screen_or_title_bar() {
    // PdfCraft had a Discord button in the title bar and an ArtCraft community card on the home
    // screen; the fork shows neither.
    let h = harness(|_| {});
    h.get_by_label_contains("Amaraza PDF");
    assert_eq!(h.query_all_by_label("Discord").count(), 0);
    assert_eq!(h.query_all_by_label_contains("ArtCraft").count(), 0);
    assert_eq!(h.query_all_by_label_contains("Discord").count(), 0);
}

#[test]
fn about_dialog_credits_pdfcraft_and_opens_the_project_links() {
    for l in links::LINKS {
        // With a document open, so the home screen is not on screen.
        let mut h = harness(move |app| {
            app.open_bytes("one.pdf", None, ONE_PAGE.to_vec()).unwrap();
            app.dialog = Some(Dialog::About);
        });
        assert_eq!(h.query_all_by_label(amaraza_identity::PRODUCT_NAME).count(), 2, "the logo (alt text) and the name");
        h.get_by_label(links::BASED_ON);
        h.get_by_label(l.label).click();
        h.run_steps(2);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(l.url), "{}", l.label);
    }
}

#[test]
fn help_commands_open_each_link() {
    assert_eq!(links::LINKS.first().map(|l| l.url), Some(amaraza_identity::REPOSITORY), "Amaraza PDF's own repository comes first");
    for l in links::LINKS {
        let mut h = harness(|_| {});
        assert!(h.state_mut().execute(l.command), "{}", l.command);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(l.url));
        assert_eq!(pdfcraft_engine::commands::command(l.command).unwrap().menu, Some("Help"));
    }
    for gone in ["help.discord", "help.app_page", "help.website"] {
        assert!(pdfcraft_engine::commands::command(gone).is_none(), "{gone} linked to ArtCraft");
    }
}

#[test]
fn about_dialog_has_contributors_and_models_tabs() {
    let mut h = harness(|app| app.dialog = Some(Dialog::About));
    h.get_by_label("Contributors").click();
    h.run_steps(2);
    // The owner is always in the compiled-in credits (contributors/contributors.json), shown by username.
    h.get_by_label("@echelon");
    h.get_by_label("Table").click();
    h.run_steps(2);
    h.get_by_label("PRs");
    h.get_by_label("Display name").click();
    h.run_steps(2);
    h.get_by_label("Brandon Thomas");
    h.get_by_label("Models").click();
    h.run_steps(2);
    assert!(h.query_all_by_label("Anthropic").count() >= 1);
}
