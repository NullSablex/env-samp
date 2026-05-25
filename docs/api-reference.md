# API reference

## Constants

| Constant | Value | Used by |
|---|---|---|
| `ENV_STRING` | `0` | `Env(..., ENV_STRING, ...)` |
| `ENV_INT` | `1` | `Env(..., ENV_INT, ...)` |
| `ENV_FLOAT` | `2` | `Env(..., ENV_FLOAT, ...)` |
| `ENV_BOOL` | `3` | `Env(..., ENV_BOOL, ...)` |
| `ENV_SAMP_VERSION` | string | Compile-time version of the plugin (mirrors `Cargo.toml`). |

## `Env`

```pawn
native bool:Env(const key[], dest[], type = ENV_STRING, dest_len = sizeof(dest));
```

Looks up `key` in the loaded `.env` map, converts it to `type` and writes it into `dest`.

**Parameters**

| Name | Description |
|---|---|
| `key` | Variable name to retrieve. |
| `dest` | Destination buffer. Type depends on `type`: `char[]` for `ENV_STRING`, `new var` for `ENV_INT`/`ENV_BOOL`, `new Float:var` for `ENV_FLOAT`. |
| `type` | One of `ENV_STRING`, `ENV_INT`, `ENV_FLOAT`, `ENV_BOOL`. Defaults to `ENV_STRING`. |
| `dest_len` | Size of `dest`. Auto-filled with `sizeof(dest)` when omitted. |

**Returns**

- `true` if `key` exists **and** the value parsed successfully for the requested type.
- `false` if the key is missing, the value is empty, or it fails the type conversion. In that case `dest` receives a zero-equivalent default (`0`, `0.0`, `false`, or an empty string).

**Unknown type**

If `type` is outside the supported range, the plugin logs a warning and falls back to `ENV_STRING`.

## `EnvCount`

```pawn
native EnvCount();
```

Returns the total number of variables loaded from the `.env` file at startup. Returns `0` when the file is missing, empty, or unreadable. Useful for diagnostics during `OnGameModeInit`.
