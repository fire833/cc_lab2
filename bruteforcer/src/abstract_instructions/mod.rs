use std::fmt::Debug;

use crate::encodings::{Architecture, CEncoder, SerializeAMD64MachineCode};

use self::{
    eight::EightInstruction, four::FourInstruction, single::SingleInstruction,
    sixteen::SixteenInstruction,
};

pub mod eight;
pub mod four;
pub mod single;
pub mod sixteen;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InstructionBlock {
    Single(SingleInstruction),
    Four(FourInstruction),
    Eight(EightInstruction),
    Sixteen(SixteenInstruction),
}

impl InstructionBlock {
    pub fn len(&self) -> usize {
        match self {
            InstructionBlock::Single(_) => 1,
            InstructionBlock::Four(_) => 4,
            InstructionBlock::Eight(_) => 8,
            InstructionBlock::Sixteen(_) => 16,
        }
    }
}

impl Debug for InstructionBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            InstructionBlock::Single(i) => write!(f, "{:?}", i),
            InstructionBlock::Four(i) => write!(f, "{:?}", i),
            InstructionBlock::Eight(i) => write!(f, "{:?}", i),
            InstructionBlock::Sixteen(i) => write!(f, "{:?}", i),
        }
    }
}

impl CEncoder for InstructionBlock {
    fn encode_to_c(&self, index: u32, arch: Architecture) -> String {
        match &self {
            InstructionBlock::Single(i) => i.encode_to_c(index, arch),
            InstructionBlock::Four(i) => i.encode_to_c(index, arch),
            InstructionBlock::Eight(i) => i.encode_to_c(index, arch),
            InstructionBlock::Sixteen(i) => i.encode_to_c(index, arch),
        }
    }
}

impl SerializeAMD64MachineCode for InstructionBlock {
    fn write_amd64_bytes(&self, bytes: &mut Vec<u8>) {
        match &self {
            InstructionBlock::Single(i) => i.write_amd64_bytes(bytes),
            InstructionBlock::Four(i) => i.write_amd64_bytes(bytes),
            InstructionBlock::Eight(i) => i.write_amd64_bytes(bytes),
            InstructionBlock::Sixteen(i) => i.write_amd64_bytes(bytes),
        }
    }
}

impl Into<Vec<SingleInstruction>> for InstructionBlock {
    fn into(self) -> Vec<SingleInstruction> {
        match self {
            InstructionBlock::Single(i) => return vec![i],
            InstructionBlock::Four(i) => return i.into(),
            InstructionBlock::Eight(i) => return i.into(),
            InstructionBlock::Sixteen(i) => return i.into(),
        }
    }
}
