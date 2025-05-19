//! Plugin infrastructure for FluxLang.
//!
//! Plugins can be registered with [`register`] and executed with [`run_all`].
//! Tests should call [`clear_plugins`] to remove any previously registered
//! plugins so state does not leak between test cases.

use crate::syntax::ast::Program;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub trait Plugin: Send + Sync {
    fn run(&self, program: &mut Program);
}

pub mod example;

static PLUGINS: Lazy<Mutex<Vec<Box<dyn Plugin>>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn register(plugin: Box<dyn Plugin>) {
    PLUGINS.lock().unwrap().push(plugin);
}

/// Remove all registered plugins.
///
/// Mainly used by tests to ensure plugins registered in one test do not
/// affect others.
pub fn clear_plugins() {
    PLUGINS.lock().unwrap().clear();
}

pub fn run_all(program: &mut Program) {
    for plugin in PLUGINS.lock().unwrap().iter() {
        plugin.run(program);
    }
}

/// Register built-in plugins used during development.
pub fn register_default_plugins() {
    register(Box::new(example::DumpAstPlugin));
}
