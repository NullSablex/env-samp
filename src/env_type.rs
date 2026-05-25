//! Native parameter type tag and typed value parsers for each variant.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvType {
    String,
    Int,
    Float,
    Bool,
}

#[derive(Debug, Clone, Copy)]
pub struct UnknownEnvType(pub i32);

impl std::fmt::Display for UnknownEnvType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown env type: {}", self.0)
    }
}

impl TryFrom<i32> for EnvType {
    type Error = UnknownEnvType;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::String),
            1 => Ok(Self::Int),
            2 => Ok(Self::Float),
            3 => Ok(Self::Bool),
            other => Err(UnknownEnvType(other)),
        }
    }
}

pub fn parse_int(value: &str) -> Option<i32> {
    value.trim().parse().ok()
}

pub fn parse_float(value: &str) -> Option<f32> {
    value.trim().parse().ok()
}

pub fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Some(true),
        "false" | "0" | "no" | "off" => Some(false),
        _ => None,
    }
}
