use crate::encodings::{Architecture, CEncoder, SerializeAMD64MachineCode};

use super::{
    eight::EightInstruction, four::FourInstruction, single::SingleInstruction, InstructionBlock,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SixteenInstruction {
    pub value1: EightInstruction,
    pub value2: EightInstruction,
}

impl SixteenInstruction {
    pub const fn new(value1: EightInstruction, value2: EightInstruction) -> Self {
        Self { value1, value2 }
    }

    pub fn new_from_instr(instrs: Vec<InstructionBlock>) -> Option<SixteenInstruction> {
        let mut full_vec = vec![];

        for blk in instrs.iter() {
            let mut cloned: Vec<SingleInstruction> = blk.clone().into();
            full_vec.append(&mut cloned);
        }

        if full_vec.len() != 16 {
            return None;
        }

        Some(SixteenInstruction::new(
            EightInstruction::new(
                FourInstruction::new(full_vec[0], full_vec[1], full_vec[2], full_vec[3]),
                FourInstruction::new(full_vec[4], full_vec[5], full_vec[6], full_vec[7]),
            ),
            EightInstruction::new(
                FourInstruction::new(full_vec[8], full_vec[9], full_vec[10], full_vec[11]),
                FourInstruction::new(full_vec[12], full_vec[13], full_vec[14], full_vec[15]),
            ),
        ))
    }
}

impl CEncoder for SixteenInstruction {
    fn encode_to_c(&self, _index: u32, _arch: Architecture) -> String {
        format!("")
    }
}

impl SerializeAMD64MachineCode for SixteenInstruction {
    fn write_amd64_bytes(&self, _bytes: &mut Vec<u8>) {}
}

impl Into<Vec<SingleInstruction>> for SixteenInstruction {
    fn into(self) -> Vec<SingleInstruction> {
        let mut vec1: Vec<SingleInstruction> = self.value1.into();
        let mut vec2: Vec<SingleInstruction> = self.value2.into();
        vec1.append(&mut vec2);

        return vec1;
    }
}
