# .env format

The parser follows the de-facto `.env` convention: one `KEY=VALUE` per line, with quoting, comments and escapes.

## Basic rules

- Empty lines and lines starting with `#` are ignored.
- Lines starting with `export ` are skipped (they are not assignments).
- The first `=` on the line separates the key from the value. Trailing `=` characters are part of the value.
- Whitespace around the key and around the value is trimmed.
- Duplicate keys: **last one wins**.

```bash
APP_ENV=production
APP_DEBUG=false

MYSQL_HOST = 127.0.0.1
MYSQL_PORT = 3306
EMPTY_VALUE =
```

## Quoting

| Quote style | Behavior |
|---|---|
| `'single'` | Literal — no escape sequences are interpreted. |
| `"double"` | Supports `\n`, `\r`, `\t`, `\"`, `\\`, plus literal pass-through for any other escape. |
| unquoted | Trimmed; an inline `#` preceded by whitespace starts a comment. |

```bash
LITERAL_TEXT = 'text with literal \n and \t'
SERVER_NAME = "My SA:MP Server"
MOTD = "Line 1\nLine 2"
WINDOWS_PATH = "C:\\samp\\plugins"
```

## Inline comments

A `#` only starts a comment when preceded by whitespace. Without preceding whitespace it is part of the value.

```bash
WITH_INLINE_COMMENT = value # this comment is ignored
HASH_WITHOUT_SPACE = value#this_is_part_of_the_value
```

## Encoding

The file must be valid UTF-8. A leading BOM (`U+FEFF`) is stripped automatically. Files larger than 1 MiB are rejected with a warning.

## Full example

[`examples/env`](https://github.com/NullSablex/env_samp/blob/master/examples/env) in the repository walks through every syntactic feature shown above in a single annotated file — copy it to `.env`, edit the values, and you're set.
