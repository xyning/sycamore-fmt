pub type Error = Box<dyn std::error::Error>;

#[cfg(windows)]
pub const NEW_LINE: &'static str = "\r\n";
#[cfg(not(windows))]
pub const NEW_LINE: &'static str = "\n";

pub fn split_by_span(s: &str, span: proc_macro2::Span) -> String {
    split_by_ln_col(
        s,
        span.start().line,
        span.start().column,
        span.end().line,
        span.end().column,
    )
}

pub fn split_by_ln_col(
    s: &str,
    ln_start: usize,
    col_start: usize,
    ln_end: usize,
    col_end: usize,
) -> String {
    let ff = s.lines().collect::<Vec<_>>();
    let ff = ff.split_at(ln_end).0;
    let mut ff = ff
        .split_at(ln_start - 1)
        .1
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>();
    let m = ff.last_mut().unwrap();
    *m = m.chars().collect::<Vec<_>>()[..col_end]
        .iter()
        .collect::<String>();
    let m = ff.first_mut().unwrap();
    *m = m.chars().collect::<Vec<_>>()[col_start..]
        .iter()
        .collect::<String>();
    ff.join(NEW_LINE)
}

pub trait Shift {
    fn shift(&self, l: usize) -> String;
    fn unshift(&self, l: usize) -> String;
}

impl<T: AsRef<str>> Shift for T {
    fn unshift(&self, l: usize) -> String {
        self.as_ref()
            .lines()
            .into_iter()
            .map(|f| f.strip_prefix(&"    ".repeat(l)).unwrap_or(f))
            .collect::<Vec<_>>()
            .join(NEW_LINE)
    }
    fn shift(&self, l: usize) -> String {
        self.as_ref()
            .lines()
            .into_iter()
            .map(|f| [&"    ".repeat(l), f].concat())
            .collect::<Vec<_>>()
            .join(NEW_LINE)
    }
}
