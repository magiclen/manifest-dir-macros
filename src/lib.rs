/*!
# Manifest Dir Macros

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

// The `tuple` feature lets these macros above support to input nested literal string tuples, which is useful when you want to use these macros inside a `macro_rule!` macro and concatenate with other literal strings.
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
*/

mod functions;
mod join_builder;

use std::{env, path::PathBuf};

use functions::*;
use join_builder::*;
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
#[cfg(feature = "mime_guess")]
use quote::quote;
use syn::parse_macro_input;

static MANIFEST_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let s = env::var_os("CARGO_MANIFEST_DIR").expect("we need CARGO_MANIFEST_DIR");

    #[cfg(all(windows, feature = "replace-separator"))]
    let s = beautify_windows_path_os(s).expect("a UTF8-encodable CARGO_MANIFEST_DIR");

    PathBuf::from(s)
});

/// Allows input an absolute path, or a relative path. If a relative path is input, it will be relative to the CARGO_MANIFEST_DIR (a directory where your `Cargo.toml` located). Returns an absolute path.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = syn::parse_macro_input!(input as JoinBuilder).into();

    let p =
        if original_path.is_absolute() { original_path } else { MANIFEST_DIR.join(original_path) };

    output_path(p)
}

/// Allows input an absolute path, or a relative path. (multiple components are supported) If a relative path is input, it will be relative to the CARGO_MANIFEST_DIR (a directory where your `Cargo.toml` located). Returns an absolute path, and it must exist.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn exist_path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = parse_macro_input!(input as JoinBuilder).into();

    let p =
        if original_path.is_absolute() { original_path } else { MANIFEST_DIR.join(original_path) };

    if p.exists() {
        output_path(p)
    } else {
        compile_error_not_exist(p)
    }
}

/// Allows input an absolute path, or a relative path. If a relative path is input, it will be relative to the CARGO_MANIFEST_DIR (a directory where your `Cargo.toml` located). Returns an absolute path, and it must be an existing directory.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn directory_path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = parse_macro_input!(input as JoinBuilder).into();

    let p =
        if original_path.is_absolute() { original_path } else { MANIFEST_DIR.join(original_path) };

    if p.is_dir() {
        output_path(p)
    } else {
        compile_error_not_directory(p)
    }
}

/// Allows input an absolute path, or a relative path. If a relative path is input, it will be relative to the CARGO_MANIFEST_DIR (a directory where your `Cargo.toml` located). Returns an absolute path, and it must not be an existing directory.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn not_directory_path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = parse_macro_input!(input as JoinBuilder).into();

    let p =
        if original_path.is_absolute() { original_path } else { MANIFEST_DIR.join(original_path) };

    if p.metadata().map(|m| !m.is_dir()).unwrap_or(false) {
        output_path(p)
    } else {
        compile_error_directory(p)
    }
}

/// Allows input an absolute path, or a relative path. If a relative path is input, it will be relative to the CARGO_MANIFEST_DIR (a directory where your `Cargo.toml` located). Returns an absolute path, and it must be an existing file.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn file_path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = parse_macro_input!(input as JoinBuilder).into();

    let p =
        if original_path.is_absolute() { original_path } else { MANIFEST_DIR.join(original_path) };

    if p.is_file() {
        output_path(p)
    } else {
        compile_error_not_file(p)
    }
}

/// Allows input a relative path. It will be relative to the CARGO_MANIFEST_DIR (a directory where your `Cargo.toml` located). Returns an absolute path.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn relative_path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = parse_macro_input!(input as JoinBuilder).into();

    if original_path.is_relative() {
        output_path(MANIFEST_DIR.join(original_path))
    } else {
        compile_error_not_relative(original_path)
    }
}

/// Allows input a relative path. It will be relative to the CARGO_MANIFEST_DIR (a directory where your `Cargo.toml` located). Returns an absolute path, and it must exist.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn exist_relative_path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = parse_macro_input!(input as JoinBuilder).into();

    if original_path.is_relative() {
        let p = MANIFEST_DIR.join(original_path);

        if p.exists() {
            output_path(p)
        } else {
            compile_error_not_exist(p)
        }
    } else {
        compile_error_not_relative(original_path)
    }
}

/// Allows input a relative path. It will be relative to the CARGO_MANIFEST_DIR (a directory where your `Cargo.toml` located). Returns an absolute path, and it must be a directory.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn directory_relative_path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = parse_macro_input!(input as JoinBuilder).into();

    if original_path.is_relative() {
        let p = MANIFEST_DIR.join(original_path);

        if p.is_dir() {
            output_path(p)
        } else {
            compile_error_not_directory(p)
        }
    } else {
        compile_error_not_relative(original_path)
    }
}

/// Allows input a relative path. It will be relative to the CARGO_MANIFEST_DIR (a directory where your `Cargo.toml` located). Returns an absolute path, and it must not be a directory.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn not_directory_relative_path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = parse_macro_input!(input as JoinBuilder).into();

    if original_path.is_relative() {
        let p = MANIFEST_DIR.join(original_path);

        if p.metadata().map(|m| !m.is_dir()).unwrap_or(false) {
            output_path(p)
        } else {
            compile_error_directory(p)
        }
    } else {
        compile_error_not_relative(original_path)
    }
}

/// Allows input a relative path. It will be relative to the CARGO_MANIFEST_DIR (a directory where your `Cargo.toml` located). Returns an absolute path, and it must be a file.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn file_relative_path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = parse_macro_input!(input as JoinBuilder).into();

    if original_path.is_relative() {
        let p = MANIFEST_DIR.join(original_path);

        if p.is_file() {
            output_path(p)
        } else {
            compile_error_not_file(p)
        }
    } else {
        compile_error_not_relative(original_path)
    }
}

/// Allows input a absolute path. Checks and returns the absolute path.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn absolute_path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = parse_macro_input!(input as JoinBuilder).into();

    if original_path.is_absolute() {
        output_path(original_path)
    } else {
        compile_error_not_absolute(original_path)
    }
}

/// Allows input a absolute path. Checks whether it exists and returns the absolute path.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn exist_absolute_path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = parse_macro_input!(input as JoinBuilder).into();

    if original_path.is_absolute() {
        if original_path.exists() {
            output_path(original_path)
        } else {
            compile_error_not_exist(original_path)
        }
    } else {
        compile_error_not_absolute(original_path)
    }
}

/// Allows input a absolute path. Checks whether it is a directory and returns the absolute path.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn directory_absolute_path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = parse_macro_input!(input as JoinBuilder).into();

    if original_path.is_absolute() {
        if original_path.is_dir() {
            output_path(original_path)
        } else {
            compile_error_not_directory(original_path)
        }
    } else {
        compile_error_not_absolute(original_path)
    }
}

/// Allows input a absolute path. Checks whether it is not a directory and returns the absolute path.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn not_directory_absolute_path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = parse_macro_input!(input as JoinBuilder).into();

    if original_path.is_absolute() {
        if original_path.metadata().map(|m| !m.is_dir()).unwrap_or(false) {
            output_path(original_path)
        } else {
            compile_error_directory(original_path)
        }
    } else {
        compile_error_not_absolute(original_path)
    }
}

/// Allows input a absolute path. Checks whether it is a file and returns the absolute path.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn file_absolute_path(input: TokenStream) -> TokenStream {
    let original_path: PathBuf = parse_macro_input!(input as JoinBuilder).into();

    if original_path.is_absolute() {
        if original_path.is_file() {
            output_path(original_path)
        } else {
            compile_error_not_file(original_path)
        }
    } else {
        compile_error_not_absolute(original_path)
    }
}

/// Gets the file name for other purposes. If there is no file name, the default value will be used, or a compile error will be shown.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn get_file_name(input: TokenStream) -> TokenStream {
    let jb = parse_macro_input!(input as JoinBuilderNoBeautifyWithDefaultValue);

    match jb.0.file_name() {
        Some(file_name) => output_os_str(file_name),
        None => match jb.1 {
            Some(expr) => output_expr(&expr),
            None => compile_error(format!("The path {:?} has no file name", jb.0)),
        },
    }
}

/// Gets the file stem for other purposes. If there is no file stem, the default value will be used, or a compile error will be shown.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn get_file_stem(input: TokenStream) -> TokenStream {
    let jb = parse_macro_input!(input as JoinBuilderNoBeautifyWithDefaultValue);

    match jb.0.file_stem() {
        Some(file_stem) => output_os_str(file_stem),
        None => match jb.1 {
            Some(expr) => output_expr(&expr),
            None => compile_error(format!("The path {:?} has no file stem", jb.0)),
        },
    }
}

/// Gets the file extension for other purposes. If there is no file extension, the default value will be used, or a compile error will be shown.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn get_extension(input: TokenStream) -> TokenStream {
    let jb = parse_macro_input!(input as JoinBuilderNoBeautifyWithDefaultValue);

    match jb.0.extension() {
        Some(extension) => output_os_str(extension),
        None => match jb.1 {
            Some(expr) => output_expr(&expr),
            None => compile_error(format!("The path {:?} has no file extension", jb.0)),
        },
    }
}

/// Gets the parent for other purposes. If there is no parent, the default value will be used, or a compile error will be shown.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn get_parent(input: TokenStream) -> TokenStream {
    let jb = parse_macro_input!(input as JoinBuilderWithDefaultValue);

    match jb.0.parent() {
        Some(parent) => output_path(parent),
        None => match jb.1 {
            Some(expr) => output_expr(&expr),
            None => compile_error(format!("The path {:?} has no parent", jb.0)),
        },
    }
}

#[cfg(feature = "mime_guess")]
/// Guesses the mime type by the path. If the guess fails, the default value will be used, or a compile error will be shown.
///
/// Multiple components can be input by using commas to separate them.
#[proc_macro]
pub fn mime_guess(input: TokenStream) -> TokenStream {
    let jb = parse_macro_input!(input as JoinBuilderNoBeautifyWithDefaultValue);

    match jb
        .0
        .extension()
        .and_then(|ext| ext.to_str())
        .and_then(|ext| mime_guess::from_ext(ext).first())
        .map(|mime| mime.to_string())
    {
        Some(mime) => {
            let code = quote! {
                #mime
            };

            code.into()
        },
        None => match jb.1 {
            Some(expr) => output_expr(&expr),
            None => {
                compile_error(format!("The path {:?} can not be guessed for its mime type", jb.0))
            },
        },
    }
}
