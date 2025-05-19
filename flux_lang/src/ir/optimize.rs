use super::IrModule;

/// Trait implemented by optimization passes.
pub trait Pass {
    fn name(&self) -> &str;
    fn run(&self, ir: &mut IrModule);
}

#[derive(Default)]
pub struct PassManager {
    passes: Vec<Box<dyn Pass>>,
}

impl PassManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_pass<P: Pass + 'static>(&mut self, pass: P) {
        self.passes.push(Box::new(pass));
    }

    pub fn run(&self, ir: &mut IrModule) {
        for pass in &self.passes {
            log::debug!("running pass: {}", pass.name());
            pass.run(ir);
        }
    }
}

struct ConstantFolding;

impl Pass for ConstantFolding {
    fn name(&self) -> &str {
        "constant-folding"
    }

    fn run(&self, _ir: &mut IrModule) {
        // TODO: actual constant folding
    }
}

struct DeadCodeElimination;

impl Pass for DeadCodeElimination {
    fn name(&self) -> &str {
        "dead-code-elimination"
    }

    fn run(&self, _ir: &mut IrModule) {
        // TODO: actual DCE
    }
}

struct TemporalFusion;

impl Pass for TemporalFusion {
    fn name(&self) -> &str {
        "temporal-fusion"
    }

    fn run(&self, _ir: &mut IrModule) {
        // TODO: temporal fusion optimization
    }
}

pub fn run_passes(ir: &mut IrModule) {
    let mut pm = PassManager::new();
    pm.add_pass(ConstantFolding);
    pm.add_pass(DeadCodeElimination);
    pm.add_pass(TemporalFusion);
    pm.run(ir);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct CountingPass(Arc<Mutex<usize>>);

    impl Pass for CountingPass {
        fn name(&self) -> &str {
            "counting"
        }

        fn run(&self, _ir: &mut IrModule) {
            *self.0.lock().unwrap() += 1;
        }
    }

    #[test]
    fn pass_manager_runs_passes() {
        let mut ir = IrModule {
            graph: petgraph::graph::Graph::new(),
        };
        let counter = Arc::new(Mutex::new(0usize));
        let mut pm = PassManager::new();
        pm.add_pass(CountingPass(counter.clone()));
        pm.run(&mut ir);
        assert_eq!(*counter.lock().unwrap(), 1);
    }
}
