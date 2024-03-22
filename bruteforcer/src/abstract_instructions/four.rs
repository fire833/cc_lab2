use crate::{abstract_instructions::InstructionBlock, encodings::CEncoder};

use super::single::SingleInstruction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FourInstruction {
    value1: SingleInstruction,
    value2: SingleInstruction,
    value3: SingleInstruction,
    value4: SingleInstruction,
}

impl FourInstruction {
    pub const fn new(
        value1: SingleInstruction,
        value2: SingleInstruction,
        value3: SingleInstruction,
        value4: SingleInstruction,
    ) -> Self {
        Self {
            value1,
            value2,
            value3,
            value4,
        }
    }

    pub fn new_from_instr(instrs: Vec<InstructionBlock>) -> Option<Self> {
        if instrs.len() != 4 {
            return None;
        }

        let (f1, f2, f3, f4);

        match instrs[0] {
            InstructionBlock::Single(s) => f1 = s,
            _ => return None,
        }

        match instrs[1] {
            InstructionBlock::Single(s) => f2 = s,
            _ => return None,
        }

        match instrs[2] {
            InstructionBlock::Single(s) => f3 = s,
            _ => return None,
        }

        match instrs[3] {
            InstructionBlock::Single(s) => f4 = s,
            _ => return None,
        }

        Some(Self::new(f1, f2, f3, f4))
    }
}

impl CEncoder for FourInstruction {
    fn encode_to_c(&self) -> String {
        format!("")
    }
}

// Returns the pairing of index to mapped value for each index.
impl Into<Vec<SingleInstruction>> for FourInstruction {
    fn into(self) -> Vec<SingleInstruction> {
        return vec![self.value1, self.value2, self.value3, self.value4];
    }
}
