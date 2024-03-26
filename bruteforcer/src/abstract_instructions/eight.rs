use crate::encodings::{CEncoder, SerializeAMD64MachineCode};

use super::{four::FourInstruction, single::SingleInstruction, InstructionBlock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EightInstruction {
    pub value1: FourInstruction,
    pub value2: FourInstruction,
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

    const fn get_first_output_index(&self) -> u32 {
        let mut smallest = u32::MAX;

        if self.value1.value1.value < smallest {
            smallest = self.value1.value1.value;
        }

        if self.value1.value2.value < smallest {
            smallest = self.value1.value2.value;
        }

        if self.value1.value3.value < smallest {
            smallest = self.value1.value3.value;
        }

        if self.value1.value4.value < smallest {
            smallest = self.value1.value4.value;
        }

        if self.value2.value1.value < smallest {
            smallest = self.value2.value1.value;
        }

        if self.value2.value2.value < smallest {
            smallest = self.value2.value2.value;
        }

        if self.value2.value3.value < smallest {
            smallest = self.value2.value3.value;
        }

        if self.value2.value4.value < smallest {
            smallest = self.value2.value4.value;
        }

        smallest
    }

    const fn get_permute_mask(&self) -> (i64, i64, i64, i64) {
        let first_out = self.get_first_output_index();
        let mut i: (i64, i64, i64, i64) = (0, 0, 0, 0);

        i.0 |= ((self.value1.value2.value + first_out) as i64) << 32;
        i.0 |= (self.value1.value1.value + first_out) as i64;
        i.1 |= ((self.value1.value4.value + first_out) as i64) << 32;
        i.1 |= (self.value1.value3.value + first_out) as i64;
        i.2 |= ((self.value2.value2.value + first_out) as i64) << 32;
        i.2 |= (self.value2.value1.value + first_out) as i64;
        i.3 |= ((self.value2.value4.value + first_out) as i64) << 32;
        i.3 |= (self.value2.value3.value + first_out) as i64;

        i
    }
}

impl CEncoder for EightInstruction {
    fn encode_to_c(&self, index: u32) -> String {
        let smallest = self.get_first_output_index();
        let mask = self.get_permute_mask();

        format!(
            "  __m256 valin{} = {{in[{}], in[{}], in[{}], in[{}], in[{}], in[{}], in[{}], in[{}]}};
  static const __m256i mask{} = {{{}, {}, {}, {}}};
  __m256 valout{} = _mm256_permutevar8x32_ps(valin{}, mask{});
  _mm256_storeu_ps(&out[{}], valout{});
",
            index,
            self.value1.value1.index,
            self.value1.value2.index,
            self.value1.value3.index,
            self.value1.value4.index,
            self.value2.value1.index,
            self.value2.value2.index,
            self.value2.value3.index,
            self.value2.value4.index,
            index,
            mask.0,
            mask.1,
            mask.2,
            mask.3,
            index,
            index,
            index,
            smallest,
            index,
        )
    }
}

impl SerializeAMD64MachineCode for EightInstruction {
    fn write_amd64_bytes(&self, bytes: &mut Vec<u8>) {}
}

impl Into<Vec<SingleInstruction>> for EightInstruction {
    fn into(self) -> Vec<SingleInstruction> {
        let mut vec1: Vec<SingleInstruction> = self.value1.into();
        let mut vec2: Vec<SingleInstruction> = self.value2.into();
        vec1.append(&mut vec2);

        return vec1;
    }
}
