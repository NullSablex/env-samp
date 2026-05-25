# env_samp

Environment variables (`.env`) plugin for SA-MP and Open Multiplayer, written entirely in Rust. Loads a `.env` file at startup and exposes its values to Pawn through a single typed native.

The same `.so` / `.dll` runs on SA-MP and on Open Multiplayer — natively as a component (recommended) or via legacy mode. See [Installation](installation.md) for both registration paths.

## Where to start

| Goal | Path |
|---|---|
| First time here | [Installation](installation.md) → [Usage](usage.md) |
| Learning the file syntax | [.env format](dotenv-format.md) |
| Quick lookup | [API reference](api-reference.md) |

## Minimal example

```pawn
#include <a_samp>
#include <env_samp>

public OnGameModeInit()
{
    new host[64];
    if (Env("MYSQL_HOST", host))
    {
        printf("[env_samp] MYSQL_HOST=%s", host);
    }

    new port;
    if (Env("MYSQL_PORT", port, ENV_INT))
    {
        printf("[env_samp] MYSQL_PORT=%d", port);
    }

    return 1;
}
```
