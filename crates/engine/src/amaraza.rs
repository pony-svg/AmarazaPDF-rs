//! Amaraza PDF fork declarations inside the engine (docs/amaraza/adr/0001-puntos-de-enganche.md).
//!
//! Data only: the fork's commands and Action Wizard steps, as the upstream registry declares its
//! own. What they do lives in the `amaraza-ext` crate and in the fork modules of the frontends.
//! The upstream files only carry the hooks marked `ADR-0001`.
//!
//! A7: this is glue inside an upstream crate, so clippy applies the root's complexity threshold
//! here, not 15 (it has no per-module threshold). Keep branching logic in `amaraza-ext`.

#![warn(clippy::cognitive_complexity)]

use crate::commands::{CommandSpec, Needs};
use crate::{DocId, EditError, Session};

/// Prefix of every fork command id.
pub const COMMAND_PREFIX: &str = "amaraza.";

/// The fork's top-level menu.
pub const MENU: &str = "Amaraza";

/// The fork's commands, appended to the upstream's in [`crate::commands::COMMANDS`]. No keyboard
/// shortcuts, so they never compete with the upstream's.
pub const COMMANDS: &[CommandSpec] = &[
    // The "hola Amaraza" demonstration (ADR-0001 §5).
    CommandSpec { id: "amaraza.hello", label: "Hola, Amaraza", menu: Some(MENU), shortcut: None, needs: Needs::Nothing, in_text: true, icon: "hand" },
];

const EMPTY: CommandSpec = CommandSpec { id: "", label: "", menu: None, shortcut: None, needs: Needs::Nothing, in_text: true, icon: "" };

/// `upstream` followed by [`COMMANDS`].
///
/// Only called from the `COMMANDS` const in `commands.rs`, so it runs at compile time: a wrong
/// `N` or an out-of-range index stops the build and can't fail at run time.
pub(crate) const fn commands<const N: usize>(upstream: &[CommandSpec]) -> [CommandSpec; N] {
    assert!(N == upstream.len() + COMMANDS.len(), "N must count both lists");
    let mut out = [EMPTY; N];
    let mut i = 0;
    while i < N {
        out[i] = if i < upstream.len() { upstream[i] } else { COMMANDS[i - upstream.len()] };
        i += 1;
    }
    out
}

/// What the demonstration step writes in the step log.
pub const HELLO_LOG: &str = "Hola, Amaraza";

/// A fork step of the Action Wizard, wrapped in [`crate::actions::Step::Amaraza`].
#[derive(Clone, Debug, PartialEq)]
pub enum Step {
    /// The demonstration step: notes a greeting in the step log and changes nothing.
    Hello,
}

impl Step {
    /// Every fork step, with an empty argument where it takes one (for the step picker).
    pub fn all() -> Vec<Step> {
        vec![Step::Hello]
    }

    /// A stable id (agents, saved actions). Never rename one: saved actions refer to it.
    pub fn id(&self) -> &'static str {
        match self {
            Step::Hello => "amaraza_hello",
        }
    }

    /// The label people see (Spanish, AR-D4).
    pub fn label(&self) -> &'static str {
        match self {
            Step::Hello => "Saludo de Amaraza",
        }
    }

    /// The step's text argument, if it takes one.
    pub fn arg(&self) -> Option<&str> {
        match self {
            Step::Hello => None,
        }
    }

    pub fn arg_mut(&mut self) -> Option<&mut String> {
        match self {
            Step::Hello => None,
        }
    }
}

/// Apply a fork step inside [`crate::actions::run_on`]. A step that needs context the action
/// can't carry (a certificate) is refused here with an actionable error and runs through
/// `amaraza-ext` instead (ADR-0001 §4).
pub(crate) fn apply(_s: &mut Session, _id: DocId, step: &Step, log: &mut Vec<String>) -> Result<Option<DocId>, EditError> {
    match step {
        Step::Hello => log.push(HELLO_LOG.to_string()),
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::actions::{self, Action};

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
    fn commands_are_the_upstreams_then_the_forks() {
        let all = crate::commands::COMMANDS;
        let n = super::COMMANDS.len();
        assert_eq!(all.first().map(|c| c.id), Some("file.open"), "the upstream's list comes first, unchanged");
        assert!(all.len() > n);
        let tail = &all[all.len() - n..];
        assert!(tail.iter().zip(super::COMMANDS).all(|(a, b)| a.id == b.id), "the fork's commands come last");
        for c in super::COMMANDS {
            assert!(c.id.starts_with(super::COMMAND_PREFIX), "{}", c.id);
            assert!(c.shortcut.is_none(), "{}: fork commands take no shortcut", c.id);
        }
    }

    #[test]
    fn fork_steps_go_through_the_upstream_step() {
        let hello = actions::Step::Amaraza(super::Step::Hello);
        assert_eq!(actions::Step::from_id("amaraza_hello", ""), Some(hello.clone()));
        assert_eq!(actions::Step::all().last(), Some(&hello), "the fork's steps come after the upstream's");
        assert_eq!(hello.id(), "amaraza_hello");
        assert_eq!(hello.label(), "Saludo de Amaraza");
        assert_eq!(hello.arg(), None);
        assert_eq!(hello.clone().arg_mut(), None);
        for s in super::Step::all() {
            assert!(s.id().starts_with("amaraza_"), "{}", s.id());
        }
    }

    #[test]
    fn the_hello_step_runs_in_an_action_and_changes_nothing() {
        let action =
            Action { name: "Saludo".into(), description: String::new(), steps: vec![actions::Step::Amaraza(super::Step::Hello)], builtin: false };
        let r = actions::run_on(&action, "dos.pdf", Arc::new(fixture(2)), |_, _| {}).unwrap();
        assert_eq!(r.log, vec![super::HELLO_LOG.to_string()]);
        let mut s = crate::Session::new();
        let id = s.open("dos.pdf", None, r.bytes, None).unwrap();
        assert_eq!(s.get(id).map(|d| d.info.pages.len()), Some(2));
    }
}
