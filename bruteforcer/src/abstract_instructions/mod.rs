use crate::encodings::CEncoder;

use self::{
    eight::EightInstruction, four::FourInstruction, single::SingleInstruction,
    sixteen::SixteenInstruction,
};

pub mod eight;
pub mod four;
pub mod single;
pub mod sixteen;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl CEncoder for InstructionBlock {
    fn encode_to_c(&self) -> String {
        match &self {
            InstructionBlock::Single(i) => i.encode_to_c(),
            InstructionBlock::Four(i) => todo!(),
            InstructionBlock::Eight(i) => todo!(),
            InstructionBlock::Sixteen(i) => todo!(),
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
