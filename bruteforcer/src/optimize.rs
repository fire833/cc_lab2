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

impl Into<Vec<u32>> for FourInstruction {
    fn into(self) -> Vec<u32> {
        return vec![
            self.value1.value,
            self.value2.value,
            self.value3.value,
            self.value4.value,
        ];
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

impl Into<Vec<u32>> for EightInstruction {
    fn into(self) -> Vec<u32> {
        let mut vec1: Vec<u32> = self.value1.into();
        let mut vec2: Vec<u32> = self.value2.into();
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

impl Into<Vec<u32>> for SixteenInstruction {
    fn into(self) -> Vec<u32> {
        let mut vec1: Vec<u32> = self.value1.into();
        let mut vec2: Vec<u32> = self.value2.into();
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
        let mut set: Vec<InstructionBlock> = vec![];
        let mut refset: Vec<InstructionBlock>;

        let mut check: HashSet<u32> = HashSet::new();

        for (index, val) in self.values.iter().enumerate() {
            set.push(InstructionBlock::Single(SingleInstruction::new(
                index as u32,
                *val,
            )));
        }

        for simd_count in vec![4, 8, 16].iter() {
            let mut changed: bool = true;

            while changed {
                changed = false;
                refset = set.clone();
                for (index, _) in refset.iter().rev().skip(simd_count - 1).rev().enumerate() {
                    // If we have a self-permutation, then we can reduce into a new instruction.
                    if self.range_self_permutes(
                        &mut check,
                        index as u32,
                        index as u32 + (*simd_count as u32),
                    ) {
                        // if *simd_count == 4 {
                        //     set.insert(index, InstructionBlock::Four(self.values[index]));
                        // } else if *simd_count == 8 {
                        //     set.insert(index, InstructionBlock::Eight(self.values[index]));
                        // } else {
                        //     set.insert(index, InstructionBlock::Sixteen(self.values[index]));
                        // }

                        for _ in 0..(*simd_count as u32) {
                            set.remove(index + 1);
                        }

                        changed = true;

                        break;
                    }
                }
            }
        }

        set
    }

    // Returns whether a range of elements "self permutes" on
    // themselves, I.e. the array subset that can be optimized
    // to a single SIMD instruction.
    fn range_self_permutes(&self, set: &mut HashSet<u32>, lower: u32, upper: u32) -> bool {
        set.clear();

        // If we indices that are out of bounds, bail out.
        if upper > (self.values.len() - 1) as u32 {
            return false;
        }

        for i in lower..upper {
            set.insert(i);
        }

        for i in lower..upper {
            if !set.contains(&self.values[i as usize]) {
                return false;
            }
        }

        true
    }
}

#[test]
fn test_dp() {
    let mask = ShiftMask::new(vec![1, 2, 3, 0, 4, 7, 5, 6]);
    assert_eq!(
        mask.optimize_to_blocks(),
        vec![InstructionBlock::Eight(1, 2, 3, 0, 4, 7, 5, 6)]
    );
}
