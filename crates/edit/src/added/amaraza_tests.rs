//! Amaraza PDF regression tests for shaped text font identity (ADR-0002).

use std::collections::HashMap;

use pdfcraft_cos::Document;

use super::*;

#[test]
fn shaped_font_family_is_amaraza_pdf() {
    let mut doc = Document::new_empty();
    let mut outlines = HashMap::new();
    let boxed = |advance: f64| GlyphOutline {
        contours: vec![vec![[0.0, 0.0], [advance, 0.0], [advance, 0.5], [0.0, 0.5]]],
        width: advance,
        bbox: [0.0, 0.0, advance, 0.5],
    };
    outlines.insert((ShapedScript::Arabic, 1), boxed(0.5));
    outlines.insert((ShapedScript::Hebrew, 2), boxed(0.5));

    let arabic = ShapedCluster { script: ShapedScript::Arabic, glyphs: vec![(1, [0.0, 0.0])], advance: 0.5, text: "ب".into() };
    let hebrew = ShapedCluster { script: ShapedScript::Hebrew, glyphs: vec![(2, [0.0, 0.0])], advance: 0.5, text: "א".into() };
    // Mixed scripts: clusters.iter().all(|c| c.script == *s) is false, hitting the None branch.
    let clusters = vec![&arabic, &hebrew];

    let font_ref = type3_shaped(&mut doc, &clusters, &outlines);
    let font_obj = doc.get(font_ref);
    let font_dict = font_obj.as_dict().expect("font is dict");
    let desc_ref = font_dict.reference(b"FontDescriptor").expect("FontDescriptor ref");
    let desc_obj = doc.get(desc_ref);
    let desc = desc_obj.as_dict().expect("descriptor is dict");

    let font_family = desc.get(b"FontFamily").and_then(|f| f.as_string()).expect("FontFamily string").to_text();
    assert_eq!(font_family, amaraza_identity::SHAPED_TEXT_FONT_FAMILY);
    assert!(!font_family.contains("PdfCraft"));

    let font_name = desc.get(b"FontName").and_then(|f| f.as_name()).expect("FontName name");
    let font_name_str = String::from_utf8_lossy(font_name);
    assert_eq!(font_name_str, amaraza_identity::SHAPED_TEXT_FONT_FAMILY.replace(' ', ""));
    assert!(!font_name_str.contains(' '));
    assert!(!font_name_str.contains("PdfCraft"));
}
