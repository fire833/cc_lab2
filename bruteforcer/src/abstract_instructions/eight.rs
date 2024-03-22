use crate::encodings::CEncoder;

use super::{four::FourInstruction, single::SingleInstruction, InstructionBlock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EightInstruction {
    value1: FourInstruction,
    value2: FourInstruction,
}

impl EightInstruction {
    pub const fn new(value1: FourInstruction, value2: FourInstruction) -> Self {
        Self { value1, value2 }
    }

    pub fn new_from_instr(instrs: Vec<InstructionBlock>) -> Option<EightInstruction> {
        let mut full_vec = vec![];

        for blk in instrs.iter() {
            let mut cloned: Vec<SingleInstruction> = blk.clone().into();
            full_vec.append(&mut cloned);
        }

        if full_vec.len() != 8 {
            return None;
        }

        Some(EightInstruction::new(
            FourInstruction::new(full_vec[0], full_vec[1], full_vec[2], full_vec[3]),
            FourInstruction::new(full_vec[4], full_vec[5], full_vec[6], full_vec[7]),
        ))
    }
}

impl CEncoder for EightInstruction {
    fn encode_to_c(&self) -> String {
        format!("")
    }
}

impl Into<Vec<SingleInstruction>> for EightInstruction {
    fn into(self) -> Vec<SingleInstruction> {
        let mut vec1: Vec<SingleInstruction> = self.value1.into();
        let mut vec2: Vec<SingleInstruction> = self.value2.into();
        vec1.append(&mut vec2);

        return vec1;
    }
}
