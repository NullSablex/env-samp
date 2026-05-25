# env_samp

[![Language](https://img.shields.io/badge/Rust-2024%20edition-orange)]()
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows-blue)]()
[![Architecture](https://img.shields.io/badge/arch-i686%20(32--bit)-lightgrey)]()
[![License](https://img.shields.io/badge/license-AGPL--3.0--or--later-green)]()

Environment variables (`.env`) plugin for SA-MP and Open Multiplayer, written in Rust. Loads a `.env` file at startup and exposes the values to Pawn through a single typed native.

Designed to eliminate hardcoded credentials in Pawn code, keeping secrets out of version control and centralized in a single configuration file.

The same `.so` / `.dll` runs on SA-MP and on Open Multiplayer — natively as a component (recommended) or via legacy mode.

## Documentation

Full documentation lives at **<https://env-samp.nullsablex.com/>**:

- [Installation](https://env-samp.nullsablex.com/installation/)
- [Usage](https://env-samp.nullsablex.com/usage/)
- [.env format](https://env-samp.nullsablex.com/dotenv-format/)
- [API reference](https://env-samp.nullsablex.com/api-reference/)

The Markdown sources are in [docs/](docs/) — `mkdocs serve` from the repo root for a local preview.

## Examples

- [examples/example.pwn](examples/example.pwn) — minimal gamemode using `Env` for string/int/float/bool and `EnvCount`.
- [examples/env](examples/env) — annotated `.env` covering every syntax feature the parser accepts (quoting, escapes, inline comments, duplicates, etc.). Copy it to `.env` at the server root and edit the values.

## Quick start

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

Run the [example .env](examples/env) alongside your server and call this from `OnGameModeInit`.

## Build

Requires Rust stable with the 32-bit targets installed. On Linux:

```bash
./scripts/build-linux.sh   # builds .so + .dll into dist/
```

On Windows (Git Bash):

```bash
./scripts/build-windows.sh
```

Both scripts read `PLUGIN_NAME` from `Cargo.toml`, install missing rustup targets and produce `dist/env_samp.so` and `dist/env_samp.dll`.

## Security

- The `.env` is read **once** at startup. No runtime reload.
- Values are **never** logged. Only error/warning messages about the file itself are emitted.
- Files larger than 1 MiB are rejected.
- Add `.env` to `.gitignore`. **Never commit credentials.**
- Use [examples/env](examples/env) as a public template without real values.

## License

This project is distributed under the [GNU Affero General Public License v3.0 or later](LICENSE).

You can use, modify and redistribute it freely, provided that derivative works are released under the same license with source code available. Because it is AGPL, modifications used on servers accessible over the network must also have their source offered to the users interacting with that instance.
