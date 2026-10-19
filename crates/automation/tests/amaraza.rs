//! The Amaraza PDF fork's tools end to end (docs/amaraza/adr/0001-puntos-de-enganche.md §5.2), on
//! synthetic PDFs in a temporary root that is removed afterwards (A11).

use std::path::{Path, PathBuf};

use pdfcraft_automation::{Automation, Content, tools};
use serde_json::{Value, json};

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

/// A temporary root holding `a.pdf` (3 pages), removed when dropped.
struct Root(PathBuf);

impl Root {
    fn new(test: &str) -> Root {
        let dir = std::env::temp_dir().join(format!("amaraza-automation-{test}-{}", std::process::id()));
        // A10: ignored on purpose. The folder is usually absent (it only exists if an earlier run
        // was killed), and a real problem shows up in `create_dir_all` just below.
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.pdf"), fixture(3)).unwrap();
        Root(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for Root {
    fn drop(&mut self) {
        // A10: ignored on purpose. `drop` can't return an error, and panicking here would hide
        // the test's own result; at worst a folder stays in the temp directory.
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn auto(root: &Root) -> Automation {
    Automation::new().with_root(root.path()).unwrap()
}

fn ok(a: &mut Automation, tool: &str, args: Value) -> Value {
    match a.call(tool, &args) {
        Ok(mut c) => match c.remove(0) {
            Content::Json(v) => v,
            other => panic!("{tool}: expected JSON, got {other:?}"),
        },
        Err(e) => panic!("{tool} {args}: {e}"),
    }
}

#[test]
fn hello_without_a_document() {
    let root = Root::new("hello");
    let mut a = auto(&root);
    let r = ok(&mut a, "amaraza_hello", json!({}));
    assert_eq!(r["product"], "Amaraza PDF");
    assert_eq!(r["version"], env!("CARGO_PKG_VERSION"));
    assert!(r["pages"].is_null(), "{r}");
    assert_eq!(r["text"], format!("Hola, Amaraza PDF {}", env!("CARGO_PKG_VERSION")));
}

#[test]
fn hello_counts_the_pages_of_an_open_document() {
    let root = Root::new("hello-doc");
    let mut a = auto(&root);
    let doc = ok(&mut a, "doc_open", json!({ "path": "a.pdf" }))["doc"].as_u64().unwrap();
    let r = ok(&mut a, "amaraza_hello", json!({ "doc": doc }));
    assert_eq!(r["pages"], 3);
    assert!(r["text"].as_str().unwrap().ends_with(" · 3 páginas"), "{r}");
}

#[test]
fn hello_refuses_bad_arguments_with_an_actionable_error() {
    let root = Root::new("hello-bad");
    let mut a = auto(&root);
    let e = a.call("amaraza_hello", &json!({ "page": 1 })).unwrap_err().to_string();
    assert!(e.contains("unknown argument") && e.contains("doc"), "{e}");
    let e = a.call("amaraza_hello", &json!({ "doc": 99 })).unwrap_err().to_string();
    assert!(e.contains("no open document"), "{e}");
}

#[test]
fn the_fork_tools_come_after_the_upstreams() {
    let all = tools();
    let hello = all.iter().find(|t| t.name == "amaraza_hello").expect("amaraza_hello is listed");
    assert!(hello.read_only && !hello.destructive);
    let first = all.iter().position(|t| t.name.starts_with("amaraza_")).unwrap();
    assert!(first > 0, "the upstream's tools come first");
    assert!(all[first..].iter().all(|t| t.name.starts_with("amaraza_")), "the fork's tools come last");
}

#[test]
fn the_hello_step_runs_through_action_run() {
    let root = Root::new("hello-step");
    let mut a = auto(&root);
    let list = ok(&mut a, "action_list", json!({}));
    let steps = list["steps"].as_array().unwrap();
    assert!(steps.iter().any(|s| s["step"] == "amaraza_hello" && s["label"] == "Saludo de Amaraza" && s["takes_arg"] == false), "{list}");
    let r = ok(&mut a, "action_run", json!({ "steps": [{ "step": "amaraza_hello" }], "paths": ["a.pdf"], "folder": "out" }));
    assert_eq!(r["files"][0]["log"][0], "Hola, Amaraza", "{r}");
    assert!(root.path().join("out").join("a.pdf").is_file());
}

#[test]
fn the_hello_command_runs_through_its_tool() {
    let root = Root::new("hello-command");
    let mut a = auto(&root);
    let list = ok(&mut a, "command_list", json!({ "filter": "amaraza." }));
    let commands = list["commands"].as_array().unwrap();
    let hello = commands.iter().find(|c| c["id"] == "amaraza.hello").expect("amaraza.hello is listed");
    assert_eq!(hello["tool"], "amaraza_hello");
    assert_eq!(hello["label"], "Hola, Amaraza");
    assert_eq!(hello["menu"], "Amaraza");
    assert_eq!(hello["enabled"], true);
    let r = ok(&mut a, "command_run", json!({ "id": "amaraza.hello" }));
    assert_eq!(r["text"], format!("Hola, Amaraza PDF {}", env!("CARGO_PKG_VERSION")));
}
