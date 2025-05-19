use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use flux_lang::{
    codegen::Backend,
    compile_with_backend,
    plugins::{self, Plugin},
};

struct CountingPlugin(Arc<AtomicUsize>);

impl Plugin for CountingPlugin {
    fn run(&self, _program: &mut flux_lang::syntax::ast::Program) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn skip_default_plugins_env() {
    plugins::clear_plugins();
    let counter = Arc::new(AtomicUsize::new(0));
    plugins::register(Box::new(CountingPlugin(counter.clone())));
    std::env::set_var("FLUX_SKIP_DEFAULT_PLUGINS", "1");
    compile_with_backend("", Backend::Llvm).unwrap();
    std::env::remove_var("FLUX_SKIP_DEFAULT_PLUGINS");
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}
