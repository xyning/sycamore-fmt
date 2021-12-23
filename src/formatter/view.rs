use crate::formatter::*;
use crate::sycamore_macro::view::*;
use crate::utils::*;
use proc_macro2::*;
use quote::*;
use std::cell::*;
use syn::spanned::Spanned;

pub fn fmt_stream(raw_src: &str, ts: TokenStream) -> Result<String, Error> {
    let x: HtmlRoot = syn::parse2(ts.clone())?;
    fmt_tree(raw_src, x.children).map(|x| match x.join(NEW_LINE).trim().lines().count() {
        0 => format!("view! {{}}"),
        1 => format!("view! {{ {} }}", x[0].trim()),
        _ => format!(
            "view! {{{nl}{}{nl}}}",
            x.join(NEW_LINE).trim().shift(1),
            nl = NEW_LINE
        ),
    })
}

fn fmt_tree(raw_src: &str, vec: Vec<HtmlTree>) -> Result<Vec<String>, Error> {
    let mut lines = Vec::with_capacity(1024);
    let current_line = RefCell::new(String::with_capacity(256));

    let push = |s: String| {
        current_line.borrow_mut().push_str(&s);
        current_line.borrow_mut().push_str(" ");
    };
    let mut new_line = || {
        lines.push(current_line.borrow().trim().to_string());
        *current_line.borrow_mut() = String::with_capacity(256);
    };

    for tree in vec {
        let str = match tree {
            HtmlTree::Component(c) => {
                let path = split_by_span(raw_src, c.path.span());
                let args = c
                    .args
                    .iter()
                    .map(|f| rust::fmt_str(&split_by_span(raw_src, f.span()), false))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", ");
                format!("{}({})", path, args)
            }
            HtmlTree::Element(e) => {
                use attributes::*;

                let tag = e.tag_name.to_string();
                let attr = e
                    .attributes
                    .ok_or::<Error>("".into())
                    .and_then(|f| {
                        Ok(f.attributes
                            .iter()
                            .map(|f| {
                                let attr_name = match &f.ty {
                                    AttributeType::Str { name } => name.to_string(),
                                    AttributeType::Bool { name } => name.to_string(),
                                    AttributeType::DangerouslySetInnerHtml => {
                                        "dangerously_set_inner_html".to_string()
                                    }
                                    AttributeType::Event { event } => format!("on:{}", event),
                                    AttributeType::Bind { prop } => format!("bind:{}", prop),
                                    AttributeType::Ref => "ref".to_string(),
                                };
                                let attr_value =
                                    rust::fmt_str(&split_by_span(raw_src, f.expr.span()), false)?;
                                Ok::<_, Error>(format!("{}={}", attr_name, attr_value))
                            })
                            .collect::<Result<Vec<_>, _>>()?
                            .join(", "))
                    })
                    .map(|f| format!("({})", f))
                    .unwrap_or("".to_string());
                let children = e
                    .children
                    .ok_or::<Error>("".into())
                    .and_then(|f| {
                        Ok(fmt_tree(raw_src, f.body)?
                            .iter()
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(NEW_LINE))
                    })
                    .map(|f| f.shift(1))
                    .map(|f| match f.trim().lines().count() {
                        0 => "{ }".to_string(),
                        1 => format!("{{ {} }}", f.trim()),
                        _ => format!("{{{nl}{}{nl}}}", f, nl = NEW_LINE),
                    })
                    .unwrap_or("".to_string());
                format!("{}{} {}", tag, attr, children)
            }
            HtmlTree::Splice(s) => {
                let le = split_by_span(raw_src, s.expr.span());
                format!("({})", rust::fmt_str(&le, false)?)
            }
            HtmlTree::Text(t) => t.to_token_stream().to_string(),
        };
        push(str);
        new_line();
    }

    new_line();
    Ok(lines)
}
