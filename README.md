Manifest Dir Macros
====================

[![CI](https://github.com/magiclen/manifest-dir-macros/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/manifest-dir-macros/actions/workflows/ci.yml)

This crate provides function-like macros to check or operate paths relative to CARGO_MANIFEST_DIR at compile time.

## Examples

```rust
#[macro_use] extern crate manifest_dir_macros;

println!(path!("Cargo.toml"));
println!(path!("src/lib.rs"));
println!(path!("src", "lib.rs"));
println!(path!("src", "lib.rs", "/bin"));
println!(path!("/usr"));

println!(exist_path!("Cargo.toml"));
println!(directory_path!("src"));
println!(not_directory_path!("Cargo.toml"));
println!(file_path!("Cargo.toml"));

println!(relative_path!("Cargo.toml"));
println!(directory_relative_path!("src"));
println!(not_directory_relative_path!("Cargo.toml"));
println!(file_relative_path!("Cargo.toml"));
```

## Crates.io

https://crates.io/crates/manifest-dir-macros

## Documentation

https://docs.rs/manifest-dir-macros

## License

[MIT](LICENSE)
