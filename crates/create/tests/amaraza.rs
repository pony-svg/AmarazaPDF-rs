//! Amaraza PDF regression tests for pdfcraft-create.

use pdfcraft_create::from_text;

#[test]
fn producer_is_amaraza_pdf() {
    let doc = from_text("title", "hello", (612.0, 792.0), 12.0).unwrap();
    let info_ref = doc.trailer().reference(b"Info").unwrap();
    let info = doc.get(info_ref);
    let producer = info.as_dict().unwrap().get(b"Producer").unwrap().as_string().unwrap().to_text();
    assert_eq!(producer, amaraza_identity::PRODUCT_NAME);
    assert!(!producer.contains("PdfCraft"));
}
