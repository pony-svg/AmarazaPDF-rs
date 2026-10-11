//! The "hola Amaraza" demonstration (ADR-0001 §5): a read-only greeting that exercises every hook
//! (command, tool, menu, translation layer and Action Wizard step).

use pdfcraft_engine::{DocId, EditError, Session};

/// What [`greeting`] reports.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Greeting {
    pub product: &'static str,
    pub version: &'static str,
    /// The page count of the document asked about, if any.
    pub pages: Option<usize>,
}

impl Greeting {
    /// The text people see (Spanish, AR-D4): "Hola, Amaraza PDF 0.5.0", plus the page count when
    /// there is a document.
    pub fn text(&self) -> String {
        let base = format!("Hola, {} {}", self.product, self.version);
        match self.pages {
            Some(1) => format!("{base} · 1 página"),
            Some(n) => format!("{base} · {n} páginas"),
            None => base,
        }
    }
}

/// The greeting, for the open document `doc` if given. Changes nothing.
pub fn greeting(session: &Session, doc: Option<DocId>) -> Result<Greeting, EditError> {
    let pages = match doc {
        Some(id) => Some(session.get(id).ok_or(EditError::NoDocument)?.info.pages.len()),
        None => None,
    };
    Ok(Greeting { product: amaraza_identity::PRODUCT_NAME, version: env!("CARGO_PKG_VERSION"), pages })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

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

    #[test]
    fn greets_without_a_document() {
        let g = greeting(&Session::new(), None).unwrap();
        assert_eq!(g.product, "Amaraza PDF");
        assert_eq!(g.pages, None);
        assert_eq!(g.text(), format!("Hola, Amaraza PDF {}", env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn counts_the_pages_of_an_open_document() {
        let mut s = Session::new();
        let id = s.open("tres.pdf", None, Arc::new(fixture(3)), None).unwrap();
        let g = greeting(&s, Some(id)).unwrap();
        assert_eq!(g.pages, Some(3));
        assert!(g.text().ends_with(" · 3 páginas"), "{}", g.text());
        let one = s.open("una.pdf", None, Arc::new(fixture(1)), None).unwrap();
        assert!(greeting(&s, Some(one)).unwrap().text().ends_with(" · 1 página"));
    }

    #[test]
    fn a_document_that_is_not_open_is_an_error() {
        let err = greeting(&Session::new(), Some(DocId(999))).unwrap_err();
        assert!(matches!(err, EditError::NoDocument), "{err:?}");
    }
}
