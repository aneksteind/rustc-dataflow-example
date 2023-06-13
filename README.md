# rustc-dataflow-example

## Running

1. set your toolchain to nightly, e.g. on linux something like `rustup default nightly-x86_64-unknown-linux-gnu`
2. test with `cargo run -- test/test.rs -L "$(rustc --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/lib" --crate-type rlib`
