use std::fmt::Debug;

use crate::encodings::{Architecture, CEncoder, SerializeAMD64MachineCode};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SingleInstruction {
    pub index: u32,
    pub value: u32,
}

impl SingleInstruction {
    pub const fn new(index: u32, value: u32) -> Self {
        Self { index, value }
    }
}

impl Debug for SingleInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.index, self.value)
    }
}

impl CEncoder for SingleInstruction {
    fn encode_to_c(&self, _index: u32, _arch: Architecture) -> String {
        format!("  out[{}] = in[{}];\n", self.index, self.value)
    }
}

impl SerializeAMD64MachineCode for SingleInstruction {
    fn write_amd64_bytes(&self, _bytes: &mut Vec<u8>) {}
}

impl From<(u32, u32)> for SingleInstruction {
    fn from(value: (u32, u32)) -> Self {
        Self {
            index: value.0,
            value: value.1,
        }
    }
}
