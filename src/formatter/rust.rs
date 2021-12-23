use crate::rustfmt::*;
use crate::utils::*;
use crate::*;
use proc_macro2::*;
use quote::*;

pub fn fmt_str(s: &str, is_file: bool) -> std::result::Result<String, Error> {
    let formatted_src = rustfmt(&s, is_file)?;
    let ts = if is_file {
        let file: syn::File = syn::parse_str(&formatted_src)?;
        file.to_token_stream()
    } else {
        let file: syn::Expr = syn::parse_str(&formatted_src)?;
        file.to_token_stream()
    };

    let macros = extract_macros_from_ts(ts);
    if macros.is_empty() {
        return Ok(formatted_src);
    }

    let formatted_macros = macros
        .iter()
        .map(|f| Ok::<_, Error>((f, f.fmt(&formatted_src)?)))
        .collect::<Result<Vec<_>, _>>()?;

    let mut result_str = String::with_capacity(s.len());

    let mut line_iter = formatted_src.lines().peekable();

    let mut cursor = LineColumn { line: 1, column: 0 };
    let mut current_line = line_iter.next().map(|f| f.chars().collect::<Vec<_>>());

    for (macro_group, macro_string) in formatted_macros {
        // copy contents before macro
        while cursor.line < macro_group.start.line {
            result_str.push_str(
                &current_line
                    .unwrap()
                    .split_at(cursor.column)
                    .1
                    .iter()
                    .collect::<String>(),
            );
            result_str.push_str(NEW_LINE);

            current_line = line_iter.next().map(|f| f.chars().collect::<Vec<_>>());
            cursor.line = cursor.line + 1;
            cursor.column = 0;
        }

        assert!(cursor.line == macro_group.start.line);
        result_str.push_str(
            &current_line
                .as_ref()
                .unwrap()
                .split_at(macro_group.start.column)
                .0
                .split_at(cursor.column)
                .1
                .iter()
                .collect::<String>(),
        );

        // push formatted macro
        let current_line_str = current_line.as_ref().unwrap().iter().collect::<String>();
        let base_shift = (current_line_str.len() - current_line_str.trim_start().len()) / 4;
        let macro_str = &macro_string.shift(base_shift);
        result_str.push_str(&macro_str.trim());

        // skip old macro
        let mut wrapped = Some(current_line_str.as_str());
        while cursor.line < macro_group.end.line {
            cursor.line = cursor.line + 1;
            wrapped = line_iter.next();
        }
        current_line = wrapped.map(|f| f.chars().collect::<Vec<_>>());

        cursor.column = macro_group.end.column;
    }

    // copy remaining contents
    loop {
        if let Some(ref cli) = current_line {
            result_str.push_str(&cli.split_at(cursor.column).1.iter().collect::<String>());
            result_str.push_str(NEW_LINE);
            current_line = line_iter.next().map(|f| f.chars().collect::<Vec<_>>());
            cursor.line = cursor.line + 1;
            cursor.column = 0;
        } else {
            break;
        }
    }

    Ok(rustfmt(result_str.trim(), is_file)?)
}

fn extract_macros_from_ts(ts: TokenStream) -> Vec<MacroGroup> {
    let mut macro_groups = vec![];
    let tt_vec = ts.into_iter().collect::<Vec<_>>();
    if tt_vec.is_empty() {
        return macro_groups;
    }

    let mut iter = tt_vec.iter().peekable();
    while let Some(tt) = iter.next() {
        match (tt, iter.peek()) {
            (TokenTree::Ident(i), Some(TokenTree::Punct(p))) if p.as_char() == '!' => {
                let ident = i.clone();
                iter.next(); // skip '!'
                let group = match &iter.next().unwrap() {
                    TokenTree::Group(g) => g.clone(),
                    _ => continue, // not a macro
                };

                let op = ident.span().start();
                let ed = group.span().end();
                let ty = MacroType::try_from(ident.to_string().as_str()).ok();

                if let Some(r#type) = ty {
                    macro_groups.push(MacroGroup {
                        group,
                        r#type,
                        start: op,
                        end: ed,
                    });
                }
            }
            (TokenTree::Group(t), _) => {
                let mut macro_group = extract_macros_from_ts(t.stream());
                macro_groups.append(&mut macro_group);
            }
            _ => {}
        }
    }
    macro_groups
}
