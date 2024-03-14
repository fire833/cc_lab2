use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionBlock {
    Single(u32),
    Four(u32),
    Eight(u32),
    Sixteen(u32),
}

/// A shiftmask wrapper struct.
pub struct ShiftMask {
    /// The array of values of this permutation mask (ie our input).
    values: Vec<u32>,
}

impl ShiftMask {
    fn new(values: Vec<u32>) -> Self {
        let mut form = vec![];
        for v in values.iter() {
            form.push(InstructionBlock::Single(*v));
        }

        Self { values }
    }

    fn optimize_to_blocks(&self) -> Vec<InstructionBlock> {
        // check through all 4 blocks
        let mut set: Vec<InstructionBlock> = vec![];
        let mut refset: Vec<InstructionBlock>;

        let mut check: HashSet<u32> = HashSet::new();

        for val in self.values.iter() {
            set.push(InstructionBlock::Single(*val));
        }

        for simd_count in vec![4, 8, 16].iter() {
            refset = set.clone();

            for (index, _) in refset.iter().rev().skip(simd_count - 1).rev().enumerate() {
                // If we have a self-permutation, then we can reduce into a new instruction.
                if self.range_self_permutes(
                    &mut check,
                    index as u32,
                    index as u32 + (*simd_count as u32),
                ) {
                    if *simd_count == 4 {
                        set.insert(index, InstructionBlock::Four(self.values[index]));
                    } else if *simd_count == 8 {
                        set.insert(index, InstructionBlock::Eight(self.values[index]));
                    } else {
                        set.insert(index, InstructionBlock::Sixteen(self.values[index]));
                    }

                    for _ in 0..(*simd_count as u32) {
                        set.remove(index + 1);
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
        vec![InstructionBlock::Four(0), InstructionBlock::Four(4)]
    );
}
