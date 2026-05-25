# Changelog

All notable changes to this project are documented in this file.

Format inspired by [Keep a Changelog](https://keepachangelog.com/). Versioning follows [Semantic Versioning](https://semver.org/). Older entries live under [`changelog/`](changelog/).

## [1.0.0] — 2026/05/25

Initial public release. Built on top of [rust-samp v3.0.0](https://github.com/NullSablex/rust-samp/releases/tag/v3.0.0). The same `.so` / `.dll` loads on SA-MP and on Open Multiplayer (native component or legacy mode).

### Added

- **Universal binary.** A single artifact runs on SA-MP and on Open Multiplayer. Open Multiplayer auto-loads it as a native component when dropped into the `components/` folder (no `config.json` entry needed — the folder itself is the registration), or in legacy mode when dropped into `plugins/` and declared under `pawn.legacy_plugins` in `config.json`.
- Natives `Env(key, dest, type, dest_len)` and `EnvCount()` covering `ENV_STRING`, `ENV_INT`, `ENV_FLOAT` and `ENV_BOOL`. Unknown type values fall back to `ENV_STRING` with a warning.
- `.env` parser with single/double quoting, escape sequences (`\n`, `\r`, `\t`, `\\`, `\"`), inline `#` comments (require preceding whitespace), `export ` skip, BOM stripping and duplicate-key "last wins" semantics.
- `ENV_SAMP_VERSION` constant in `env_samp.inc` — string with the plugin version, auto-generated from `CARGO_PKG_VERSION` by `build.rs`.
- Logger with banner and detailed log file at `logs/env.log`. Configurable level (0–4); values are never logged.
- Hard limit of 1 MiB on the `.env` file size — rejected with a warning if exceeded.
- MkDocs Material documentation site published at <https://env-samp.nullsablex.com/>.
- CI: `build`/`test`/`clippy`/`fmt`/`audit`/`coverage` jobs (`.github/workflows/rust.yml`), release workflow with tag-vs-`Cargo.toml` sanity check and auto-extracted changelog section (`.github/workflows/release.yml`), MkDocs strict build + GitHub Pages deploy (`.github/workflows/docs.yml`).

### Security

- The `.env` is read **once** at startup. No runtime reload.
- Values are **never** logged. Only error/warning messages about the file itself are emitted.
- Distributed under the [GNU Affero General Public License v3.0 or later](LICENSE) — modifications used on servers accessible over the network must have their source offered to the users interacting with that instance.
