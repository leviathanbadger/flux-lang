use flux_lang::plugins;
use quickcheck::quickcheck;

quickcheck! {
    fn addition_commutes(x: i32, y: i32) -> bool {
        x.wrapping_add(y) == y.wrapping_add(x)
    }

    fn compile_never_panics(input: String) -> bool {
        plugins::clear_plugins();
        std::panic::catch_unwind(|| {
            let _ = flux_lang::compile(&input);
        })
        .is_ok()
    }
}
