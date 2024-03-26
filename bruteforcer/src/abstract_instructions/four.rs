use crate::{
    abstract_instructions::InstructionBlock,
    encodings::{Architecture, CEncoder, SerializeAMD64MachineCode},
};

use super::single::SingleInstruction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FourInstruction {
    pub value1: SingleInstruction,
    pub value2: SingleInstruction,
    pub value3: SingleInstruction,
    pub value4: SingleInstruction,
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

    const fn get_first_output_index(&self) -> u32 {
        let mut smallest = u32::MAX;

        if self.value1.value < smallest {
            smallest = self.value1.value;
        }

        if self.value2.value < smallest {
            smallest = self.value2.value;
        }

        if self.value3.value < smallest {
            smallest = self.value3.value;
        }

        if self.value4.value < smallest {
            smallest = self.value4.value;
        }

        smallest
    }

    const fn get_permute_mask(&self) -> i32 {
        let first_out = self.get_first_output_index();
        let mut i: i32 = 0;

        i |= ((self.value4.value - first_out) as i32) << 6;
        i |= ((self.value3.value - first_out) as i32) << 4;
        i |= ((self.value2.value - first_out) as i32) << 2;
        i |= (self.value1.value - first_out) as i32;
        i
    }
}

impl CEncoder for FourInstruction {
    fn encode_to_c(&self, index: u32, arch: Architecture) -> String {
        match &arch {
            Architecture::Amd64 => {
                let smallest: u32 = self.get_first_output_index();
                let mask: i32 = self.get_permute_mask();

                format!(
                    "  __m128 valin{} = {{in[{}], in[{}], in[{}], in[{}]}};
          __m128 valout{} = _mm_permute_ps(valin{}, {});
          _mm_store_ps(&out[{}], valout{});
        ",
                    index,
                    self.value1.index,
                    self.value2.index,
                    self.value3.index,
                    self.value4.index,
                    index,
                    index,
                    mask,
                    smallest,
                    index
                )
            }
            Architecture::Arm => format!(""),
        }
    }
}

impl SerializeAMD64MachineCode for FourInstruction {
    fn write_amd64_bytes(&self, _bytes: &mut Vec<u8>) {}
}

// Returns the pairing of index to mapped value for each index.
impl Into<Vec<SingleInstruction>> for FourInstruction {
    fn into(self) -> Vec<SingleInstruction> {
        return vec![self.value1, self.value2, self.value3, self.value4];
    }
}
