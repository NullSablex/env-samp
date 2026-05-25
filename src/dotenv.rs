use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum DotenvError {
    NotFound,
    NotRegularFile,
    TooLarge { max_bytes: usize, actual_bytes: u64 },
    Io(std::io::Error),
    Utf8,
}

impl std::fmt::Display for DotenvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "not found"),
            Self::NotRegularFile => write!(f, "is not a regular file"),
            Self::TooLarge {
                max_bytes,
                actual_bytes,
            } => write!(
                f,
                "exceeds the size limit ({actual_bytes} > {max_bytes} bytes)"
            ),
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::Utf8 => write!(f, "contains invalid UTF-8"),
        }
    }
}

impl std::error::Error for DotenvError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl DotenvError {
    pub const fn is_warning(&self) -> bool {
        matches!(
            self,
            Self::NotFound | Self::NotRegularFile | Self::TooLarge { .. }
        )
    }
}

pub fn load_dotenv_file(
    path: &Path,
    max_bytes: usize,
) -> Result<HashMap<String, String>, DotenvError> {
    let meta = match fs::metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Err(DotenvError::NotFound),
        Err(e) => return Err(DotenvError::Io(e)),
    };
    if !meta.is_file() {
        return Err(DotenvError::NotRegularFile);
    }

    if meta.len() > max_bytes as u64 {
        return Err(DotenvError::TooLarge {
            max_bytes,
            actual_bytes: meta.len(),
        });
    }

    let bytes = fs::read(path).map_err(DotenvError::Io)?;
    if bytes.len() > max_bytes {
        return Err(DotenvError::TooLarge {
            max_bytes,
            actual_bytes: bytes.len() as u64,
        });
    }

    let content = String::from_utf8(bytes).map_err(|_| DotenvError::Utf8)?;
    Ok(parse_dotenv_str(&content))
}

pub fn parse_dotenv_str(input: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for raw_line in strip_bom(input).lines() {
        if let Some((key, value)) = parse_line(raw_line) {
            map.insert(key, value);
        }
    }
    map
}

fn parse_line(raw: &str) -> Option<(String, String)> {
    let line = raw.strip_suffix('\r').unwrap_or(raw).trim();
    if line.is_empty() || line.starts_with('#') || line.starts_with("export ") {
        return None;
    }

    let (key, rest) = line.split_once('=')?;
    let key = key.trim();
    if key.is_empty() {
        return None;
    }

    Some((key.to_string(), parse_value(rest.trim())))
}

fn parse_value(rest: &str) -> String {
    if let Some(inner) = strip_quoted(rest, '\'') {
        return inner.to_string();
    }
    if let Some(inner) = strip_quoted(rest, '"') {
        return unescape_double_quoted(inner);
    }
    strip_inline_comment(rest).trim_end().to_string()
}

fn strip_quoted(s: &str, quote: char) -> Option<&str> {
    if s.len() >= 2 && s.starts_with(quote) && s.ends_with(quote) {
        Some(&s[1..s.len() - 1])
    } else {
        None
    }
}

fn strip_inline_comment(rest: &str) -> &str {
    let mut prev_ws = false;
    for (i, ch) in rest.char_indices() {
        if ch == '#' && prev_ws {
            return &rest[..i];
        }
        prev_ws = ch.is_whitespace();
    }
    rest
}

fn strip_bom(input: &str) -> &str {
    input.strip_prefix('\u{FEFF}').unwrap_or(input)
}

fn unescape_double_quoted(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }

        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple() {
        let m = parse_dotenv_str("A=1\n");
        assert_eq!(m.get("A").unwrap(), "1");
    }

    #[test]
    fn trims_key_and_value() {
        let m = parse_dotenv_str("  A  =  1  \n");
        assert_eq!(m.get("A").unwrap(), "1");
    }

    #[test]
    fn ignores_full_line_comment_and_blank() {
        let m = parse_dotenv_str("\n# comment\nA=1\n");
        assert_eq!(m.len(), 1);
    }

    #[test]
    fn inline_comment_requires_whitespace() {
        let m = parse_dotenv_str("A=1#not_comment\nB=1 # comment\n");
        assert_eq!(m.get("A").unwrap(), "1#not_comment");
        assert_eq!(m.get("B").unwrap(), "1");
    }

    #[test]
    fn single_quotes_are_literal() {
        let m = parse_dotenv_str("A='1\\n2'\n");
        assert_eq!(m.get("A").unwrap(), "1\\n2");
    }

    #[test]
    fn double_quotes_unescape() {
        let m = parse_dotenv_str("A=\"1\\n2\\t\\\"x\\\"\\\\\"\n");
        assert_eq!(m.get("A").unwrap(), "1\n2\t\"x\"\\");
    }

    #[test]
    fn rejects_export_prefix() {
        let m = parse_dotenv_str("export A=1\n");
        assert_eq!(m.get("A"), None);
        assert!(m.is_empty());
    }

    #[test]
    fn duplicates_last_wins() {
        let m = parse_dotenv_str("A=1\nA=2\n");
        assert_eq!(m.get("A").unwrap(), "2");
    }

    #[test]
    fn strips_bom_prefix() {
        let m = parse_dotenv_str("\u{FEFF}A=1\nB=2\n");
        assert_eq!(m.get("A").unwrap(), "1");
        assert_eq!(m.get("B").unwrap(), "2");
    }

    #[test]
    fn ignores_invalid_line_without_equals() {
        let m = parse_dotenv_str("A=1\nINVALID\nB=2\n");
        assert_eq!(m.len(), 2);
        assert_eq!(m.get("B").unwrap(), "2");
    }
}
