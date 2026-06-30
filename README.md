# lwmacct-260629-rust-helloworld

A small Rust arithmetic library and CLI for learning Cargo and crates.io publishing.

## API

```rust
use lwmacct_260629_rust_helloworld::{add, divide};

assert_eq!(add(1.0, 2.0), 3.0);
assert_eq!(divide(6.0, 3.0).unwrap(), 2.0);
```

## CLI

```bash
cargo run -- add 1 2
cargo run -- sub 1 2
cargo run -- mul 3 2
cargo run -- div 3 2
```

Example output:

```text
3
```

## Publish Check

```bash
cargo test
cargo publish --dry-run
```
