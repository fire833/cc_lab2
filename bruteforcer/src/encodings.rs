use std::fmt::Display;

/// Trait to encode objects to machine code.
pub trait SerializeAMD64MachineCode {
    fn write_amd64_bytes(&self, bytes: &mut Vec<u8>);
}

/// The architectures that are supported by these encoders.
#[derive(Debug, Clone, Copy)]
pub enum Architecture {
    Amd64,
    Arm,
}

impl Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Architecture::Amd64 => write!(f, "amd64"),
            Architecture::Arm => write!(f, "arm"),
        }
    }
}

impl TryFrom<String> for Architecture {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "amd64" => return Ok(Self::Amd64),
            "arm" => return Ok(Self::Arm),
            _ => return Err("please provide a valid architecture".to_string()),
        }
    }
}

/// Trait to encode instruction blocks to C code strings.
pub trait CEncoder {
    fn encode_to_c(&self, index: u32, arch: Architecture) -> String;
}
