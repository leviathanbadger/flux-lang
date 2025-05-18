use crate::syntax::ast::Program;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub trait Plugin: Send + Sync {
    fn run(&self, program: &mut Program);
}

static PLUGINS: Lazy<Mutex<Vec<Box<dyn Plugin>>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn register(plugin: Box<dyn Plugin>) {
    PLUGINS.lock().unwrap().push(plugin);
}

pub fn run_all(program: &mut Program) {
    for plugin in PLUGINS.lock().unwrap().iter() {
        plugin.run(program);
    }
}
