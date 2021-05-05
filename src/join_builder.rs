use std::path::PathBuf;

use crate::syn::parse::{Parse, ParseStream};
use crate::syn::LitStr;

#[cfg(feature = "tuple")]
use crate::syn::{Expr, Lit};

#[cfg(feature = "tuple")]
use crate::syn::spanned::Spanned;

#[cfg(feature = "tuple")]
use crate::quote::ToTokens;

pub struct JoinBuilder(pub PathBuf);
pub struct JoinBuilderNoBeautify(pub PathBuf);

#[cfg(not(feature = "tuple"))]
fn parse(input: ParseStream, _beautify: bool) -> Result<PathBuf, syn::Error> {
    let s = input.parse::<LitStr>()?.value();

    #[cfg(all(windows, feature = "replace-separator"))]
    let s = if _beautify {
        crate::functions::beautify_windows_path(s)
    } else {
        s
    };

    let mut path = PathBuf::from(s);

    loop {
        if input.is_empty() {
            return Ok(path);
        }

        input.parse::<Token!(,)>()?;

        if input.is_empty() {
            return Ok(path);
        }

        let s = input.parse::<LitStr>()?.value();

        #[cfg(all(windows, feature = "replace-separator"))]
        let s = crate::functions::beautify_windows_path(s);

        path.push(s);
    }
}

#[cfg(feature = "tuple")]
fn handle_expr(expr: Expr, path: &mut PathBuf, _beautify: bool) -> Result<(), syn::Error> {
    match expr {
        Expr::Lit(lit) => {
            if let Lit::Str(s) = lit.lit {
                let s = s.value();

                #[cfg(all(windows, feature = "replace-separator"))]
                let s = if _beautify {
                    crate::functions::beautify_windows_path(s)
                } else {
                    s
                };

                path.push(s);
            } else {
                return Err(syn::Error::new(lit.span(), "not a literal string"));
            }
        }
        Expr::Tuple(tuple) => {
            for expr in tuple.elems {
                handle_expr(expr, path, _beautify)?;
            }
        }
        Expr::Group(group) => {
            // In order to use the `expr` matcher in this macro. I don't know why it ends up here.
            let expr = syn::parse2::<Expr>(group.expr.into_token_stream())?;

            return handle_expr(expr, path, _beautify);
        }
        Expr::Paren(paren) => {
            let expr = syn::parse2::<Expr>(paren.expr.into_token_stream())?;

            return handle_expr(expr, path, _beautify);
        }
        _ => {
            return Err(syn::Error::new(
                expr.span(),
                "not a literal string or a literal string tuple",
            ));
        }
    }

    Ok(())
}

#[cfg(feature = "tuple")]
fn parse(input: ParseStream, _beautify: bool) -> Result<PathBuf, syn::Error> {
    if input.is_empty() {
        // to hint developers that they must input some arguments
        let _ = input.parse::<LitStr>()?;
    }

    let mut path = PathBuf::new();

    while !input.is_empty() {
        let expr = input.parse::<Expr>()?;

        handle_expr(expr, &mut path, _beautify)?;

        if input.lookahead1().peek(Token!(,)) {
            input.parse::<Token!(,)>()?;
        } else {
            break;
        }
    }

    Ok(path)
}

impl Parse for JoinBuilder {
    #[inline]
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(JoinBuilder(parse(input, true)?))
    }
}

impl Parse for JoinBuilderNoBeautify {
    #[inline]
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(JoinBuilderNoBeautify(parse(input, false)?))
    }
}

impl From<JoinBuilder> for PathBuf {
    #[inline]
    fn from(jb: JoinBuilder) -> Self {
        jb.0
    }
}

impl From<JoinBuilderNoBeautify> for PathBuf {
    #[inline]
    fn from(jb: JoinBuilderNoBeautify) -> Self {
        jb.0
    }
}
