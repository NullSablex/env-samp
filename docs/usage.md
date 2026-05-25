# Usage

The plugin exposes two natives: `Env` to read values and `EnvCount` to inspect how many were loaded.

## Reading by type

`Env(key, dest, type, dest_len)` writes the value of `key` into `dest`, converted to `type`. It returns `true` if the key exists *and* the value parsed successfully for that type.

```pawn
new host[64];
if (Env("MYSQL_HOST", host))
{
    printf("MYSQL_HOST=%s", host);
}

new port;
if (Env("MYSQL_PORT", port, ENV_INT))
{
    printf("MYSQL_PORT=%d", port);
}

new Float:rate;
if (Env("TICK_RATE", rate, ENV_FLOAT))
{
    printf("TICK_RATE=%f", rate);
}

new bool:debug;
if (Env("APP_DEBUG", debug, ENV_BOOL))
{
    printf("APP_DEBUG=%d", _:debug);
}
```

## Supported types

| Constant | Meaning |
|---|---|
| `ENV_STRING` | Default. Writes the raw value into the buffer. |
| `ENV_INT` | Parses the value with `i32::from_str` (`base 10`). |
| `ENV_FLOAT` | Parses the value with `f32::from_str`. |
| `ENV_BOOL` | Accepts (case-insensitive) `true`/`1`/`yes`/`on` → true and `false`/`0`/`no`/`off` → false. |

If the value is missing or fails to parse, `Env` writes a zero/empty default into `dest` and returns `false`.

## Counting

`EnvCount()` returns the number of variables loaded from the `.env` file. Useful for diagnostics during boot.

```pawn
printf("[env_samp] Variables loaded: %d", EnvCount());
```

## Full gamemode example

A complete Pawn script exercising all four types and `EnvCount` lives at [`examples/example.pwn`](https://github.com/NullSablex/env-samp/blob/master/examples/example.pwn) in the repository, paired with [`examples/env`](https://github.com/NullSablex/env-samp/blob/master/examples/env) as a copy-and-edit `.env` template.
