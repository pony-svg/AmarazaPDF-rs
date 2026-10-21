//! Amaraza PDF regression tests for pdfcraft-sign.

use std::sync::Arc;

use pdfcraft_cos::Document;
use pdfcraft_sign::{SignOptions, pkcs12, sign};

fn data(name: &str) -> Vec<u8> {
    std::fs::read(format!("{}/tests/data/{name}", env!("CARGO_MANIFEST_DIR"))).unwrap()
}

/// One page with text and an unsigned signature field "Approval".
fn fixture() -> Vec<u8> {
    let objs: Vec<&[u8]> = vec![
        b"<< /Type /Catalog /Pages 2 0 R /AcroForm << /Fields [6 0 R] >> >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 300 300] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> /Annots [6 0 R] >>",
        b"<< /Length 44 >>\nstream\nBT /F1 14 Tf 20 250 Td (Contract text) Tj ET\nendstream",
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
        b"<< /Type /Annot /Subtype /Widget /FT /Sig /T (Approval) /Rect [150 20 280 70] /P 3 0 R /F 4 >>",
    ];
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (i, o) in objs.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", i + 1).as_bytes());
        out.extend_from_slice(o);
        out.extend_from_slice(b"\nendobj\n");
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objs.len() + 1).as_bytes());
    for o in offsets {
        out.extend_from_slice(format!("{o:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objs.len() + 1).as_bytes());
    out
}

fn open(b: &[u8]) -> Document {
    Document::open(Arc::new(b.to_vec())).unwrap()
}

fn opts() -> SignOptions {
    SignOptions {
        page: 0,
        rect: Some([20.0, 100.0, 220.0, 150.0]),
        reason: Some("I approve this document".into()),
        location: Some("London".into()),
        date: "D:20261002120000+01'00'".into(),
        ..SignOptions::default()
    }
}

#[test]
fn signature_app_name_is_amaraza_pdf() {
    let id = pkcs12::open(&data("rsa-aes.p12"), "test").unwrap();
    let signed = sign(&open(&fixture()), &id, &opts()).unwrap();
    let doc = open(&signed);
    let root = doc.get(doc.root().unwrap());
    let acro = doc.resolve(root.as_dict().unwrap().get(b"AcroForm").unwrap());
    let fields = doc.resolve(acro.as_dict().unwrap().get(b"Fields").unwrap());
    let sig_field =
        fields.as_array().unwrap().iter().map(|f| doc.resolve(f)).find(|f| f.as_dict().unwrap().get(b"V").is_some()).expect("signed field with /V");
    let v_ref = sig_field.as_dict().unwrap().reference(b"V").unwrap();
    let v = doc.get(v_ref);
    let build = doc.resolve(v.as_dict().unwrap().get(b"Prop_Build").unwrap());
    let app = doc.resolve(build.as_dict().unwrap().get(b"App").unwrap());
    let name = app.as_dict().unwrap().name(b"Name").unwrap();
    assert_eq!(name, amaraza_identity::PRODUCT_NAME.as_bytes());
    assert!(!String::from_utf8_lossy(name).contains("PdfCraft"));
}
