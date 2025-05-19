//! Intermediate representation definitions

#![allow(clippy::module_inception)]

pub mod ir;
pub mod optimize;

pub use ir::{lower, IrModule};
pub use optimize::{run_passes, Pass, PassManager};
