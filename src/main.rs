mod formatter;
mod rustfmt;
mod sycamore_macro;
mod utils;

use crate::formatter::*;
use crate::utils::*;
use proc_macro2::*;
use std::fs::*;

fn main() {
    fn main() -> std::result::Result<(), Error> {
        let args: Vec<String> = std::env::args().collect();
        assert_eq!(args.len(), 2);

        let file_path = args[1].as_str();
        let result = rust::fmt_str(&read_to_string(file_path)?, true)?;
        write(file_path, result)?;

        Ok(())
    }
    main().unwrap();
}

#[derive(Debug, PartialEq)]
enum MacroType {
    View,
    Cloned,
}

impl TryFrom<&str> for MacroType {
    type Error = ();
    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        match s {
            "view" => Ok(Self::View),
            "cloned" => Ok(Self::Cloned),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct MacroGroup {
    group: Group,
    r#type: MacroType,
    start: LineColumn,
    end: LineColumn,
}

impl MacroGroup {
    pub fn fmt(&self, src: &str) -> Result<String, Error> {
        Ok(match self.r#type {
            MacroType::View => view::fmt_stream(src, self.group.stream())?,
            MacroType::Cloned => cloned::fmt_stream(src, self.group.stream())?,
        })
    }
}
