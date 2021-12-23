use crate::formatter::rust;
use crate::utils::*;
use proc_macro2::*;
use syn::spanned::Spanned;

pub fn fmt_stream(raw_src: &str, ts: TokenStream) -> Result<String, Error> {
    let mut iter = ts.into_iter().peekable();
    let cloned = match iter.next() {
        Some(TokenTree::Group(s)) if matches!(s.delimiter(), Delimiter::Parenthesis) => Ok({
            s.stream()
                .to_string()
                .split(",")
                .map(|s| s.trim())
                .collect::<Vec<_>>()
                .join(", ")
        }),
        _ => Err("syntax error"),
    }?;

    iter.next(); // =
    iter.next(); // >

    let span = iter.collect::<TokenStream>().span();
    let rs = rust::fmt_str(&split_by_span(raw_src, span), false)?;

    Ok(format!("cloned!(({}) => {})", cloned, rs))
}
