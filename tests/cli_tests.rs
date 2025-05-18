use assert_cmd::Command;

#[test]
fn runs_fluxc() {
    let mut cmd = Command::cargo_bin("fluxc").unwrap();
    cmd.arg("input.flux").assert().success();
}
