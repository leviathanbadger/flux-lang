Format code with `cargo fmt`.
Run `cargo clippy -- -D warnings` and `cargo test` before committing.
Use `anyhow::Result` and `anyhow::Error` for error handling unless an external crate requires another type.
Write unit tests for new functionality.
Keep functions small and clearly named.
When editing CI pipelines that use secrets or heavy resources, restrict them to the master branch; otherwise allow all branches.
