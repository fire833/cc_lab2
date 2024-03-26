use crate::encodings::{CEncoder, SerializeAMD64MachineCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleInstruction {
    pub index: u32,
    pub value: u32,
}

impl SingleInstruction {
    pub const fn new(index: u32, value: u32) -> Self {
        Self { index, value }
    }
}

impl CEncoder for SingleInstruction {
    fn encode_to_c(&self, _index: u32) -> String {
        format!("  out[{}] = in[{}];\n", self.value, self.index)
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
