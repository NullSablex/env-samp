# Installation

Drop the platform-specific binary into your server's `plugins/` folder and register it. The same binaries work on SA-MP and Open Multiplayer.

## Artifacts

Each release at [github.com/NullSablex/env_samp/releases](https://github.com/NullSablex/env_samp/releases) ships:

- `env_samp.so` — Linux i686 (`i686-unknown-linux-gnu`).
- `env_samp.dll` — Windows i686 (`i686-pc-windows-msvc`).
- `env_samp.inc` — Pawn include, identical for SA-MP and Open Multiplayer.

## SA-MP

1. Copy `env_samp.so` / `env_samp.dll` into `plugins/`.
2. Copy `env_samp.inc` into `pawno/include/`.
3. Register under `plugins=` in `server.cfg`:

```ini
plugins env_samp
```

## Open Multiplayer (native component, recommended)

Drop the binary into the server's `components/` folder. open.mp auto-discovers it on start and loads it via `ComponentEntryPoint`, with access to `ICore`, `ITimersComponent` and the other native APIs. **No `config.json` entry is required** — native components are discovered by scanning the folder, not by declaration.

## Open Multiplayer (legacy mode)

Drop the binary into `plugins/` and declare it under `pawn.legacy_plugins` in `config.json` (legacy plugins must be listed explicitly, unlike native components):

```json
{
  "pawn": {
    "legacy_plugins": ["env_samp"]
  }
}
```

## .env file

Place a `.env` file alongside your server executable. The plugin reads `./.env` on `OnGameModeInit`. See [.env format](dotenv-format.md) for the accepted syntax.

A ready-made template lives in the repository at [`examples/env`](https://github.com/NullSablex/env_samp/blob/master/examples/env) — copy it to `.env` and replace the values.

## Example gamemode

A minimal Pawn script wiring every supported type lives at [`examples/example.pwn`](https://github.com/NullSablex/env_samp/blob/master/examples/example.pwn). It's also the script used to smoke-test the plugin during development.
