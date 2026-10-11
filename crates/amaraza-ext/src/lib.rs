//! amaraza-ext — the Amaraza PDF fork's features over the engine: the headless API behind the
//! fork's commands, tools and Action Wizard steps (docs/amaraza/adr/0001-puntos-de-enganche.md).
//!
//! The engine declares the fork's commands and steps (`pdfcraft_engine::amaraza`); automation and
//! the UI call into this crate from their own fork modules. Anything with branching logic lives
//! here, where A7's threshold of 15 applies.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
// A7 (.agents/rules/01-amaraza-heredadas.md): the threshold, 15, is in this crate's clippy.toml.
#![warn(clippy::cognitive_complexity)]

pub mod hello;
