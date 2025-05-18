use std::process::Command;

#[test]
fn runs_fluxc() {
    if let Ok(exe) = std::env::var("CARGO_BIN_EXE_fluxc") {
        let status = Command::new(exe)
            .args(["compile", "examples/hello.flux"])
            .status()
            .expect("failed to run fluxc");
        assert!(status.success());
    } else {
        eprintln!("CARGO_BIN_EXE_fluxc not set; skipping test");
    }
}
