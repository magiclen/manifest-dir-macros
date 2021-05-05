extern crate proc_macro2;

use std::path::PathBuf;

use crate::syn::parse::{Parse, ParseStream};
use crate::syn::LitStr;

pub struct JoinBuilder(pub PathBuf);

impl Parse for JoinBuilder {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let s = input.parse::<LitStr>()?.value();

        #[cfg(all(windows, feature = "replace-separator"))]
        let s = crate::functions::beautify_windows_path(s);

        let mut path = PathBuf::from(s);

        loop {
            if input.is_empty() {
                return Ok(JoinBuilder(path));
            }

            input.parse::<Token!(,)>()?;

            if input.is_empty() {
                return Ok(JoinBuilder(path));
            }

            let s = input.parse::<LitStr>()?.value();

            path.push(s);
        }
    }
}

impl From<JoinBuilder> for PathBuf {
    #[inline]
    fn from(jb: JoinBuilder) -> Self {
        jb.0
    }
}
