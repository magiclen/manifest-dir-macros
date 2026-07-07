Manifest Dir Macros
====================

[![CI](https://github.com/magiclen/manifest-dir-macros/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/manifest-dir-macros/actions/workflows/ci.yml)

This crate provides function-like macros to check or operate paths relative to **CARGO_MANIFEST_DIR** at compile time.

## Features

- `replace-separator`: Enables Windows path separator replacement for path-producing macros. This feature is enabled by default.
- `tuple`: Enables nested literal string tuples as path input, which is useful when calling these macros from `macro_rules!` macros that capture `$x:expr` values.
- `mime_guess`: Enables the `mime_guess!` macro for guessing MIME types from path extensions.

This crate is a proc-macro crate and uses `std` at macro expansion time. The expanded macro output is a string literal, so it can still be used by target crates that do not use `std`.

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

println!(get_file_name!("src/lib.rs"));
println!(get_file_name!(default = "main.rs", "/"));
println!(get_file_stem!("src/lib.rs"));
println!(get_file_stem!(default = "lib", "/"));
println!(get_extension!("src/lib.rs"));
println!(get_extension!(default = "rs", "src/lib"));
println!(get_parent!("src/lib.rs"));
println!(get_parent!(default = "/home", "/"));

#[cfg(feature = "mime_guess")]
{
    println!(mime_guess!("src/lib.rs"));
    println!(mime_guess!(default = "application/octet-stream", "Cargo.lock"));
}

// The `tuple` feature lets these macros above support to input nested literal string tuples, which is useful when you want to use these macros inside a `macro_rules!` macro and concatenate with other literal strings.
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
