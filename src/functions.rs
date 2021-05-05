#[cfg(all(windows, feature = "replace-separator"))]
use std::ffi::OsString;

use std::ffi::OsStr;

use std::path::Path;

use crate::TokenStream;

#[cfg(all(windows, feature = "replace-separator"))]
// On Windows, `/` or `\` could be used as the path separator. We would prefer customarily using `/` as the separator in our hard code. This replacement is not necessary but can make the path look good.
#[inline]
pub fn beautify_windows_path(mut s: String) -> String {
    let bytes = unsafe { s.as_mut_vec() };

    for b in bytes.iter_mut() {
        if *b == b'/' {
            *b = std::path::MAIN_SEPARATOR as u8;
        }
    }

    s
}

#[cfg(all(windows, feature = "replace-separator"))]
#[inline]
pub fn beautify_windows_path_os(s: OsString) -> Result<String, OsString> {
    let s = s.into_string()?;

    Ok(beautify_windows_path(s))
}

#[inline]
pub fn compile_error<S: AsRef<str>>(s: S) -> TokenStream {
    let s = s.as_ref();

    let code = quote! {
        compile_error!(#s)
    };

    code.into()
}

#[inline]
pub fn compile_error_not_exist<P: AsRef<Path>>(p: P) -> TokenStream {
    compile_error(format!("The path {:?} does not exist", p.as_ref()))
}

#[inline]
pub fn compile_error_not_directory<P: AsRef<Path>>(p: P) -> TokenStream {
    compile_error(format!("The path {:?} is not a directory", p.as_ref()))
}

#[inline]
pub fn compile_error_directory<P: AsRef<Path>>(p: P) -> TokenStream {
    let p = p.as_ref();

    if p.exists() {
        compile_error(format!("The path {:?} is a directory", p))
    } else {
        compile_error_not_exist(p)
    }
}

#[inline]
pub fn compile_error_not_file<P: AsRef<Path>>(p: P) -> TokenStream {
    compile_error(format!("The path {:?} is not a file", p.as_ref()))
}

#[inline]
pub fn compile_error_not_relative<P: AsRef<Path>>(p: P) -> TokenStream {
    compile_error(format!("The path {:?} is not relative", p.as_ref()))
}

#[inline]
pub fn compile_error_not_absolute<P: AsRef<Path>>(p: P) -> TokenStream {
    compile_error(format!("The path {:?} is not absolute", p.as_ref()))
}

#[inline]
pub fn output_os_str<S: AsRef<OsStr>>(s: S) -> TokenStream {
    let s = s.as_ref();

    match s.to_str() {
        Some(utf8_str) => {
            let code = quote! {
                #utf8_str
            };

            code.into()
        }
        None => {
            compile_error(format!("The OsStr {:?} cannot be canonicalized to a UTF-8 string.", s))
        }
    }
}

#[inline]
pub fn output_path<P: AsRef<Path>>(p: P) -> TokenStream {
    let p = p.as_ref();

    match p.to_str() {
        Some(utf8_str) => {
            let code = quote! {
                #utf8_str
            };

            code.into()
        }
        None => {
            compile_error(format!("The path {:?} cannot be canonicalized to a UTF-8 string.", p))
        }
    }
}
