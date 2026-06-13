## Rust GitHub Template

This is a simple repository template for Rust projects. It has the basic structure plus GitHub Actions configuration for
testing on Linux, macOS, and Windows on ARM64/aarch64 and x86_64.

Additionally, linters are applied. This includes Clippy and Rustfmt for codebase, plus tests to see if any dependencies
are found to have [security risks](https://github.com/rustsec/rustsec/tree/HEAD/cargo-audit) or if [unused dependencies](https://github.com/est31/cargo-udeps) were added to the project.
