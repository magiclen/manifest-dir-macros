Manifest Dir Macros
====================

[![CI](https://github.com/magiclen/manifest-dir-macros/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/manifest-dir-macros/actions/workflows/ci.yml)

This crate provides function-like macros to check or operate paths relative to **CARGO_MANIFEST_DIR** at compile time.

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

// The `tuple` feature let these macros support to input nested literal string tuples, which is useful when you want to use these macros inside a `macro_rule!` macro and concatenate with other literal strings.
// `$x:expr` matchers can be used in these macros thus.
#[cfg(feature = "tuple")]
{
    println!(path!(("foo",)));
    println!(path!(("foo", "bar")));
    println!(path!("a", ("foo", "bar")));
    println!(path!(("foo", "bar"), "a"));
    println!(path!(("foo", "bar"), ("a", "b")));
    println!(path!(("foo", "bar", ("a", "b")), ("c", "d")));
}
```

## Crates.io

https://crates.io/crates/manifest-dir-macros

## Documentation

https://docs.rs/manifest-dir-macros

## License

[MIT](LICENSE)
