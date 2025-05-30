//! Plugin infrastructure for FluxLang.
//!
//! Plugins can be registered with [`PluginRegistry::register`] and executed with
//! [`PluginRegistry::run_all`]. Tests should call
//! [`PluginRegistry::clear`] to remove any previously registered plugins so
//! state does not leak between test cases.

use crate::syntax::ast::Program;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub trait Plugin: Send + Sync {
    fn run(&self, program: &mut Program);
}

pub mod example;

/// Registry for development plugins.
pub struct PluginRegistry(Mutex<Vec<Box<dyn Plugin>>>);

impl PluginRegistry {
    /// Register a plugin.
    pub fn register(&self, plugin: Box<dyn Plugin>) {
        self.0.lock().unwrap().push(plugin);
    }

    /// Remove all registered plugins.
    ///
    /// Mainly used by tests to ensure plugins registered in one test do not
    /// affect others.
    pub fn clear(&self) {
        self.0.lock().unwrap().clear();
    }

    /// Execute all registered plugins.
    pub fn run_all(&self, program: &mut Program) {
        for plugin in self.0.lock().unwrap().iter() {
            plugin.run(program);
        }
    }
}

pub static REGISTRY: Lazy<PluginRegistry> = Lazy::new(|| PluginRegistry(Mutex::new(Vec::new())));

/// Register built-in plugins used during development.
pub fn register_default_plugins() {
    REGISTRY.register(Box::new(example::DumpAstPlugin));
}
