use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleInstruction {
    index: u32,
    value: u32,
}

impl SingleInstruction {
    const fn new(index: u32, value: u32) -> Self {
        Self { index, value }
    }

    fn add_value_to_set(&self, set: &mut HashSet<u32>) {
        set.insert(self.value);
    }

    fn is_key_in_set(&self, set: &mut HashSet<u32>) -> bool {
        set.contains(&self.index)
    }
}

impl From<(u32, u32)> for SingleInstruction {
    fn from(value: (u32, u32)) -> Self {
        Self {
            index: value.0,
            value: value.1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FourInstruction {
    value1: SingleInstruction,
    value2: SingleInstruction,
    value3: SingleInstruction,
    value4: SingleInstruction,
}

impl FourInstruction {
    const fn new(
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
}

// Returns the pairing of index to mapped value for each index.
impl Into<Vec<SingleInstruction>> for FourInstruction {
    fn into(self) -> Vec<SingleInstruction> {
        return vec![self.value1, self.value2, self.value3, self.value4];
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EightInstruction {
    value1: FourInstruction,
    value2: FourInstruction,
}

impl EightInstruction {
    const fn new(value1: FourInstruction, value2: FourInstruction) -> Self {
        Self { value1, value2 }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SixteenInstruction {
    value1: EightInstruction,
    value2: EightInstruction,
}

impl SixteenInstruction {
    const fn new(value1: EightInstruction, value2: EightInstruction) -> Self {
        Self { value1, value2 }
    }
}

impl Into<Vec<SingleInstruction>> for SixteenInstruction {
    fn into(self) -> Vec<SingleInstruction> {
        let mut vec1: Vec<SingleInstruction> = self.value1.into();
        let mut vec2: Vec<SingleInstruction> = self.value2.into();
        vec1.append(&mut vec2);

        return vec1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionBlock {
    Single(SingleInstruction),
    Four(FourInstruction),
    Eight(EightInstruction),
    Sixteen(SixteenInstruction),
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

/// A shiftmask wrapper struct.
pub struct ShiftMask {
    /// The array of values of this permutation mask (ie our input).
    values: Vec<u32>,
}

impl ShiftMask {
    const fn new(values: Vec<u32>) -> Self {
        Self { values }
    }

    fn optimize_to_blocks(&self) -> Vec<InstructionBlock> {
        // check through all 4 blocks
        let mut set1: Vec<InstructionBlock> = vec![];
        let mut set2: Vec<InstructionBlock> = vec![];

        // only allocate one of these to maybe save on some space.
        let mut check: HashSet<u32> = HashSet::new();
        let mut store_vec: Vec<SingleInstruction> = vec![];

        for (index, val) in self.values.iter().enumerate() {
            set1.push(InstructionBlock::Single(SingleInstruction::new(
                index as u32,
                *val,
            )));
        }

        let mut output: &Vec<InstructionBlock> = &vec![];

        for simd_count in vec![4, 8, 16].iter() {
            let mut i = 0;

            while i <= set1.len() - *simd_count {
                if ShiftMask::chunk_self_permutes(
                    &mut check,
                    &mut store_vec,
                    &set1.as_slice()[i..i + *simd_count],
                ) {
                    // match *simd_count {
                    //     4 => set2.push(InstructionBlock::Four(FourInstruction::new(
                    //         set1[i],
                    //         set1[i + 1],
                    //         set1[i + 2],
                    //         set1[i + 3],
                    //     ))),
                    //     8 => {}
                    //     16 => {}
                    //     _ => {}
                    // }

                    i += *simd_count;
                } else {
                    set2.push(set1[i]);
                    i += 1;
                }
            }

            let tmp = set1.clone();
            set1 = set2;
            set2 = tmp;
            output = &set2;
        }

        output.to_vec()
    }

    // Returns whether a range of elements "self permutes" on
    // themselves, I.e. the array subset that can be optimized
    // to a single SIMD instruction.
    fn chunk_self_permutes(
        set: &mut HashSet<u32>,
        full_vec: &mut Vec<SingleInstruction>,
        chunk: &[InstructionBlock],
    ) -> bool {
        set.clear();
        full_vec.clear();

        for blk in chunk.iter() {
            let mut cloned: Vec<SingleInstruction> = blk.clone().into();
            full_vec.append(&mut cloned);
        }

        for instr in full_vec.iter() {
            instr.add_value_to_set(set);
        }

        for instr in full_vec.iter() {
            if !instr.is_key_in_set(set) {
                return false;
            }
        }

        true
    }
}

#[test]
fn test_dp() {
    let mask = ShiftMask::new(vec![1, 2, 3, 0, 4, 7, 5, 6]);
    // assert_eq!(
    //     mask.optimize_to_blocks(),
    //     vec![InstructionBlock::Eight(1, 2, 3, 0, 4, 7, 5, 6)]
    // );
}
