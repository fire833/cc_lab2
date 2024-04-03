use std::fmt::Debug;

use crate::{
    abstract_instructions::InstructionBlock,
    encodings::{Architecture, CEncoder, SerializeAMD64MachineCode},
};

use super::single::SingleInstruction;

#[derive(Clone, Copy, PartialEq, Eq)]
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

    const fn get_first_input_index(&self) -> u32 {
        let mut smallest = u32::MAX;

        if self.value1.index < smallest {
            smallest = self.value1.index;
        }

        if self.value2.index < smallest {
            smallest = self.value2.index;
        }

        if self.value3.index < smallest {
            smallest = self.value3.index;
        }

        if self.value4.index < smallest {
            smallest = self.value4.index;
        }

        smallest
    }

    const fn get_permute_mask(&self) -> u8 {
        let first_out = self.get_first_output_index();
        let mut i: u8 = 0;

        let v1 = (self.value1.value - first_out) as i32;
        let v2 = (self.value2.value - first_out) as i32;
        let v3 = (self.value3.value - first_out) as i32;
        let v4 = (self.value4.value - first_out) as i32;

        match v1 {
            0 => i |= 0,
            1 => i |= 1,
            2 => i |= 2,
            3 => i |= 3,
            _ => {}
        }

        match v2 {
            0 => i |= 0 << 2,
            1 => i |= 1 << 2,
            2 => i |= 2 << 2,
            3 => i |= 3 << 2,
            _ => {}
        }

        match v3 {
            0 => i |= 0 << 4,
            1 => i |= 1 << 4,
            2 => i |= 2 << 4,
            3 => i |= 3 << 4,
            _ => {}
        }

        match v4 {
            0 => i |= 0 << 6,
            1 => i |= 1 << 6,
            2 => i |= 2 << 6,
            3 => i |= 3 << 6,
            _ => {}
        }

        i
    }
}

impl Debug for FourInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:?}, {:?}, {:?}, {:?}]",
            self.value1, self.value2, self.value3, self.value4
        )
    }
}

impl CEncoder for FourInstruction {
    fn encode_to_c(&self, index: u32, arch: Architecture) -> String {
        match &arch {
            Architecture::Amd64 => {
                let smallest_in: u32 = self.get_first_input_index();
                let smallest_out: u32 = self.get_first_output_index();
                let mask: u8 = self.get_permute_mask();

                format!(
                    "  __m128 valin{} = _mm_maskload_ps(&in[{}], quadmask);
  __m128 valout{} = _mm_permute_ps(valin{}, {});
  _mm_maskstore_ps(&out[{}], quadmask, valout{});
",
                    index, smallest_in, index, index, mask, smallest_out, index
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
