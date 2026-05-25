#![allow(clippy::needless_pass_by_ref_mut)]
// Natives must accept `&mut self` because the `#[native]` macro and the
// `SampPlugin` trait require that signature, even when the body never mutates.

use samp::native;
use samp::prelude::*;

use crate::env_type::{self, EnvType};
use crate::logger::Logger;
use crate::plugin::EnvPlugin;

impl EnvPlugin {
    #[native(name = "Env")]
    pub fn env(
        &mut self,
        _amx: &Amx,
        key: &AmxString,
        dest: UnsizedBuffer,
        env_type_id: i32,
        dest_len: i32,
    ) -> AmxResult<bool> {
        let key = key.to_string();
        let value = self.store.get(&key);
        let buf_len = usize::try_from(dest_len).unwrap_or(0).max(1);
        let kind = resolve_env_type(&key, env_type_id);

        match kind {
            EnvType::String => write_string(dest, buf_len, value),
            EnvType::Int => Ok(write_cell(
                dest,
                buf_len,
                value.and_then(env_type::parse_int),
            )),
            EnvType::Float => Ok(write_cell(
                dest,
                buf_len,
                value
                    .and_then(env_type::parse_float)
                    .map(|f| f.to_bits().cast_signed()),
            )),
            EnvType::Bool => Ok(write_cell(
                dest,
                buf_len,
                value.and_then(env_type::parse_bool).map(i32::from),
            )),
        }
    }

    #[native(name = "EnvCount")]
    pub fn env_count(&mut self, _amx: &Amx) -> i32 {
        i32::try_from(self.store.count()).unwrap_or(i32::MAX)
    }
}

fn resolve_env_type(key: &str, env_type_id: i32) -> EnvType {
    match EnvType::try_from(env_type_id) {
        Ok(kind) => kind,
        Err(unknown) => {
            Logger::warn(&format!("Env('{key}'): {unknown}, falling back to string"));
            EnvType::String
        }
    }
}

fn write_string(dest: UnsizedBuffer, buf_len: usize, value: Option<&str>) -> AmxResult<bool> {
    dest.write_str(buf_len, value.unwrap_or(""))?;
    Ok(value.is_some())
}

fn write_cell(dest: UnsizedBuffer, buf_len: usize, parsed: Option<i32>) -> bool {
    let mut buffer = dest.into_sized_buffer(buf_len);
    buffer.set_as::<i32>(0, parsed.unwrap_or(0));
    parsed.is_some()
}
