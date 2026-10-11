//! Amaraza PDF fork tools (docs/amaraza/adr/0001-puntos-de-enganche.md): their definitions,
//! appended to the upstream's by `tools::tools`, and their dispatch from `Automation::call`.
//!
//! Tool titles and descriptions are agent-facing documentation, in English like the upstream's
//! table; the text a tool returns for people is Spanish (AR-D5).
//!
//! A7: this is glue inside an upstream crate, so clippy applies the root's complexity threshold
//! here, not 15 (it has no per-module threshold). It only declares and dispatches; the logic is
//! in `amaraza-ext`.

#![warn(clippy::cognitive_complexity)]

use serde_json::{Value, json};

use crate::tools::ToolDef;
use crate::{Args, Automation, Result, ToolError, failed};

/// Prefix of every fork tool name.
pub(crate) const PREFIX: &str = "amaraza_";

/// The fork's tools, in a stable order.
pub(crate) fn tools() -> Vec<ToolDef> {
    vec![ToolDef {
        name: "amaraza_hello",
        title: "Hello, Amaraza",
        description: "Amaraza PDF demonstration: the product name and version and, with doc, that document's page count. `text` is the greeting people see. Changes nothing.",
        input_schema: json!({
            "type": "object",
            "properties": { "doc": { "type": "integer", "minimum": 1, "description": "Document id from doc_open or doc_list (optional)." } },
            "required": [],
            "additionalProperties": false
        }),
        read_only: true,
        destructive: false,
        command: None,
    }]
}

/// Run the fork tool `name`, which starts with [`PREFIX`]. `check_args` has already accepted `a`.
pub(crate) fn call(auto: &Automation, name: &str, a: &Args) -> Result<Value> {
    match name {
        "amaraza_hello" => hello(auto, a),
        other => Err(ToolError::UnknownTool(other.into())),
    }
}

fn hello(auto: &Automation, a: &Args) -> Result<Value> {
    let doc = match a.opt_int("doc")? {
        Some(_) => Some(auto.doc(a)?.id),
        None => None,
    };
    let g = amaraza_ext::hello::greeting(&auto.session, doc).map_err(failed)?;
    Ok(json!({ "product": g.product, "version": g.version, "pages": g.pages, "text": g.text() }))
}
