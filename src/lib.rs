mod dotenv;
mod log;
mod state;

use samp::cell::string::put_in_buffer;
use samp::prelude::*;
use samp::{initialize_plugin, native};
use std::path::Path;

const DOTENV_PATH: &str = "./.env";
const DOTENV_MAX_BYTES: usize = 1024 * 1024;

#[allow(dead_code)]
const ENV_STRING: i32 = 0;
const ENV_INT: i32 = 1;
const ENV_FLOAT: i32 = 2;
const ENV_BOOL: i32 = 3;

struct Plugin;

impl SampPlugin for Plugin {
    fn on_load(&mut self) {
        let _ = log::init_logging();

        if state::is_initialized() {
            return;
        }

        let path = Path::new(DOTENV_PATH);
        match dotenv::load_dotenv_file(path, DOTENV_MAX_BYTES) {
            Ok(map) => {
                let count = map.len();
                let _ = state::init(map);
                log::info(&format!("Carregadas {count} variavel(is) de {DOTENV_PATH}"));
            }
            Err(dotenv::DotenvError::NotFound) => {
                let _ = state::init(Default::default());
                log::warn(&format!("{DOTENV_PATH} nao encontrado"));
            }
            Err(dotenv::DotenvError::NotRegularFile) => {
                let _ = state::init(Default::default());
                log::warn(&format!("{DOTENV_PATH} nao e um arquivo regular"));
            }
            Err(dotenv::DotenvError::TooLarge { max_bytes, actual_bytes }) => {
                let _ = state::init(Default::default());
                log::warn(&format!(
                    "{DOTENV_PATH} excede o limite ({actual_bytes} > {max_bytes} bytes)"
                ));
            }
            Err(dotenv::DotenvError::Io(err)) => {
                let _ = state::init(Default::default());
                log::error(&format!("Falha ao ler {DOTENV_PATH}: {err}"));
            }
            Err(dotenv::DotenvError::Utf8) => {
                let _ = state::init(Default::default());
                log::error(&format!("{DOTENV_PATH} contem UTF-8 invalido"));
            }
        }
    }
}

impl Plugin {
    #[native(name = "Env")]
    fn env(
        &mut self,
        _amx: &Amx,
        key: AmxString,
        dest: UnsizedBuffer,
        env_type: i32,
        dest_len: i32,
    ) -> AmxResult<bool> {
        let key = key.to_string();
        let value = state::get(&key);
        let buf_len = if dest_len > 0 { dest_len as usize } else { 1 };
        let mut buffer = dest.into_sized_buffer(buf_len);

        match env_type {
            ENV_INT => {
                let parsed = value.and_then(|v| v.trim().parse::<i32>().ok());
                buffer[0] = parsed.unwrap_or(0);
                Ok(parsed.is_some())
            }
            ENV_FLOAT => {
                let parsed = value.and_then(|v| v.trim().parse::<f32>().ok());
                buffer[0] = parsed.unwrap_or(0.0).to_bits() as i32;
                Ok(parsed.is_some())
            }
            ENV_BOOL => {
                let parsed = value.and_then(parse_bool);
                buffer[0] = if parsed.unwrap_or(false) { 1 } else { 0 };
                Ok(parsed.is_some())
            }
            _ => {
                let out = value.unwrap_or("");
                let truncated = if out.len() >= buf_len {
                    &out[..out.floor_char_boundary(buf_len.saturating_sub(1))]
                } else {
                    out
                };
                put_in_buffer(&mut buffer, truncated)?;
                Ok(value.is_some())
            }
        }
    }

    #[native(name = "EnvCount")]
    fn env_count(&mut self, _amx: &Amx) -> AmxResult<i32> {
        Ok(i32::try_from(state::count()).unwrap_or(i32::MAX))
    }
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Some(true),
        "false" | "0" | "no" | "off" => Some(false),
        _ => None,
    }
}

initialize_plugin!(
    natives: [Plugin::env, Plugin::env_count],
    {
        let plugin = Plugin;
        return plugin;
    }
);
