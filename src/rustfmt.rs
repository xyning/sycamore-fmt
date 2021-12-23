use crate::utils::*;

pub fn rustfmt(code: &str, new_line_ending: bool) -> Result<String, Error> {
    use std::{io::Write, process::*};
    let pre = "const _fmt:() = {\n    ();\n";
    let suf = "};\n";
    let mut proc = Command::new("rustfmt")
        .arg("--emit=stdout")
        .arg("--edition=2018")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;
    let stdin = proc.stdin.as_mut().ok_or("Unwrapping None")?;
    stdin.write_all(pre.as_bytes())?;
    stdin.write_all(code.as_bytes())?;
    stdin.write_all(suf.as_bytes())?;

    let output = proc.wait_with_output()?;

    Ok(if output.status.success() {
        let start = pre.len() + 1;
        let end = output.stdout.len() - suf.len();
        let s = std::str::from_utf8(&output.stdout[start..end])
            .unwrap()
            .to_owned();
        let s = s.unshift(1);
        let s = if new_line_ending { s + NEW_LINE } else { s };
        Ok(s)
    } else {
        Err("rustfmt err")
    }?)
}
