#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Types used to represent maps and their configuration.
//!
//! This allows us to define the bounds of a map for a game, as well as image layers to render. In
//! contrast to `Object`s, an entity should be created for each map `Layer`.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod config;
pub mod loaded;
