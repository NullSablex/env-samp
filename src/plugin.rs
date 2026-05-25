use samp::prelude::*;
use std::path::Path;

use crate::dotenv;
use crate::logger::Logger;
use crate::store::EnvStore;

pub const DOTENV_PATH: &str = "./.env";
pub const DOTENV_MAX_BYTES: usize = 1024 * 1024;

pub struct EnvPlugin {
    pub store: EnvStore,
}

impl EnvPlugin {
    pub fn new() -> Self {
        Logger::init();
        Self {
            store: EnvStore::new(),
        }
    }
}

impl SampPlugin for EnvPlugin {
    fn on_load(&mut self) {
        let path = Path::new(DOTENV_PATH);
        match dotenv::load_dotenv_file(path, DOTENV_MAX_BYTES) {
            Ok(map) => self.store.load(map),
            Err(err) if err.is_warning() => {
                Logger::warn(&format!("{DOTENV_PATH} {err}"));
            }
            Err(err) => {
                Logger::error(&format!("{DOTENV_PATH}: {err}"));
            }
        }
    }
}
