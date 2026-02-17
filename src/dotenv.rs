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
    let content = content.strip_prefix('\u{FEFF}').unwrap_or(&content);
    Ok(parse_dotenv_str(content))
}

pub fn parse_dotenv_str(input: &str) -> HashMap<String, String> {
    let input = input.strip_prefix('\u{FEFF}').unwrap_or(input);
    let mut map = HashMap::new();

    for raw_line in input.lines() {
        let raw_line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with("export ") {
            continue;
        }
        let Some((key, rest)) = line.split_once('=') else {
            continue;
        };

        let key = key.trim();
        if key.is_empty() {
            continue;
        }

        let rest = rest.trim();
        let value = if rest.len() >= 2 && rest.starts_with('\'') && rest.ends_with('\'') {
            rest[1..rest.len() - 1].to_string()
        } else if rest.len() >= 2 && rest.starts_with('"') && rest.ends_with('"') {
            unescape_double_quoted(&rest[1..rest.len() - 1])
        } else {
            let mut cut: Option<usize> = None;
            let mut prev_ws = false;
            for (i, ch) in rest.char_indices() {
                if ch == '#' && prev_ws {
                    cut = Some(i);
                    break;
                }
                prev_ws = ch.is_whitespace();
            }
            let rest = match cut {
                Some(i) => &rest[..i],
                None => rest,
            };
            rest.trim_end().to_string()
        };

        map.insert(key.to_string(), value);
    }

    map
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
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
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
