fn main() {
    // Generate parser from grammar.lalrpop
    println!("cargo:rerun-if-changed=src/syntax/grammar.lalrpop");
    lalrpop::process_root().unwrap();
}
