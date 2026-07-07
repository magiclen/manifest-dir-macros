use std::path::PathBuf;

#[cfg(feature = "tuple")]
use syn::Lit;
use syn::{
    parse::{Parse, ParseStream},
    Expr, LitStr, Token,
};

pub struct JoinBuilder(pub PathBuf);
pub struct JoinBuilderNoBeautify(pub PathBuf);

pub struct JoinBuilderWithDefaultValue(pub PathBuf, pub Option<Expr>);
pub struct JoinBuilderNoBeautifyWithDefaultValue(pub PathBuf, pub Option<Expr>);

#[cfg(not(feature = "tuple"))]
fn parse(
    input: ParseStream,
    default_value: bool,
    _beautify: bool,
) -> Result<(PathBuf, Option<Expr>), syn::Error> {
    let default_value = if default_value && input.lookahead1().peek(Token!(default)) {
        input.parse::<Token!(default)>()?;
        input.parse::<Token!(=)>()?;

        let expr = input.parse::<Expr>()?;

        input.parse::<Token!(,)>()?;

        Some(expr)
    } else {
        None
    };

    let s = input.parse::<LitStr>()?.value();

    #[cfg(all(windows, feature = "replace-separator"))]
    let s = if _beautify { crate::functions::beautify_windows_path(s) } else { s };

    let mut path = PathBuf::from(s);

    loop {
        if input.is_empty() {
            return Ok((path, default_value));
        }

        input.parse::<Token!(,)>()?;

        if input.is_empty() {
            return Ok((path, default_value));
        }

        let s = input.parse::<LitStr>()?.value();

        #[cfg(all(windows, feature = "replace-separator"))]
        let s = if _beautify { crate::functions::beautify_windows_path(s) } else { s };

        path.push(s);
    }
}

#[cfg(feature = "tuple")]
fn handle_expr(expr: Expr, path: &mut PathBuf, _beautify: bool) -> Result<(), syn::Error> {
    match expr {
        Expr::Lit(lit) => match lit.lit {
            Lit::Str(s) => {
                let s = s.value();

                #[cfg(all(windows, feature = "replace-separator"))]
                let s = if _beautify { crate::functions::beautify_windows_path(s) } else { s };

                path.push(s);
            },
            lit => return Err(syn::Error::new_spanned(lit, "not a literal string")),
        },
        Expr::Tuple(tuple) => {
            for expr in tuple.elems {
                handle_expr(expr, path, _beautify)?;
            }
        },
        Expr::Group(group) => {
            // In order to use the `expr` matcher in this macro. I don't know why it ends up here.
            return handle_expr(*group.expr, path, _beautify);
        },
        Expr::Paren(paren) => {
            return handle_expr(*paren.expr, path, _beautify);
        },
        _ => {
            return Err(syn::Error::new_spanned(
                expr,
                "not a literal string or a literal string tuple",
            ));
        },
    }

    Ok(())
}

#[cfg(feature = "tuple")]
fn parse(
    input: ParseStream,
    default_value: bool,
    _beautify: bool,
) -> Result<(PathBuf, Option<Expr>), syn::Error> {
    if input.is_empty() {
        // to hint developers that they must input some arguments
        let _ = input.parse::<LitStr>()?;
    }

    let default_value = if default_value && input.lookahead1().peek(Token!(default)) {
        input.parse::<Token!(default)>()?;
        input.parse::<Token!(=)>()?;

        let expr = input.parse::<Expr>()?;

        input.parse::<Token!(,)>()?;

        Some(expr)
    } else {
        None
    };

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

    Ok((path, default_value))
}

impl Parse for JoinBuilder {
    #[inline]
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let result = parse(input, false, true)?;

        Ok(JoinBuilder(result.0))
    }
}

impl Parse for JoinBuilderNoBeautify {
    #[inline]
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let result = parse(input, false, false)?;

        Ok(JoinBuilderNoBeautify(result.0))
    }
}

impl Parse for JoinBuilderWithDefaultValue {
    #[inline]
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let result = parse(input, true, true)?;

        Ok(JoinBuilderWithDefaultValue(result.0, result.1))
    }
}

impl Parse for JoinBuilderNoBeautifyWithDefaultValue {
    #[inline]
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let result = parse(input, true, false)?;

        Ok(JoinBuilderNoBeautifyWithDefaultValue(result.0, result.1))
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
