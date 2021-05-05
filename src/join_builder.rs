extern crate proc_macro2;

use std::path::PathBuf;

use crate::syn::parse::{Parse, ParseStream};
use crate::syn::LitStr;

#[cfg(feature = "tuple")]
use crate::syn::{Expr, Lit};

#[cfg(feature = "tuple")]
use crate::syn::spanned::Spanned;

pub struct JoinBuilder(pub PathBuf);

#[cfg(not(feature = "tuple"))]
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

            #[cfg(all(windows, feature = "replace-separator"))]
            let s = crate::functions::beautify_windows_path(s);

            path.push(s);
        }
    }
}

#[cfg(feature = "tuple")]
impl Parse for JoinBuilder {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        if input.is_empty() {
            // to hint developers that they must input some arguments
            let _ = input.parse::<LitStr>()?;
        }

        let mut path = PathBuf::new();

        while !input.is_empty() {
            let expr = input.parse::<Expr>()?;

            match expr {
                Expr::Lit(lit) => {
                    if let Lit::Str(s) = lit.lit {
                        let s = s.value();

                        #[cfg(all(windows, feature = "replace-separator"))]
                        let s = crate::functions::beautify_windows_path(s);

                        path.push(s);
                    } else {
                        return Err(syn::Error::new(lit.span(), "not a literal string"));
                    }
                }
                Expr::Tuple(tuple) => {
                    for e in tuple.elems {
                        if let Expr::Lit(lit) = e {
                            if let Lit::Str(s) = lit.lit {
                                let s = s.value();

                                #[cfg(all(windows, feature = "replace-separator"))]
                                let s = crate::functions::beautify_windows_path(s);

                                path.push(s);
                            } else {
                                return Err(syn::Error::new(lit.span(), "not a literal string"));
                            }
                        } else {
                            return Err(syn::Error::new(e.span(), "not a literal string tuple"));
                        }
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        expr.span(),
                        "not a literal string or a literal string tuple",
                    ));
                }
            }

            if input.lookahead1().peek(Token!(,)) {
                input.parse::<Token!(,)>()?;
            } else {
                break;
            }
        }

        Ok(JoinBuilder(path))
    }
}

impl From<JoinBuilder> for PathBuf {
    #[inline]
    fn from(jb: JoinBuilder) -> Self {
        jb.0
    }
}
