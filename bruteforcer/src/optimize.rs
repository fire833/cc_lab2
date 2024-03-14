use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub enum InstructionBlock {
    Single(u32),
    Four(u32, u32, u32, u32),
    Eight(u32, u32, u32, u32, u32, u32, u32, u32),
    Sixteen(
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
    ),
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

        refset = set.clone();

        // Try and merge 4 tuples
        for (index, instr) in refset.iter().rev().skip(3).enumerate() {
            // If we have a self-permutation, then we can reduce into a new instruction.
            if self.range_self_permutes(&mut check, index as u32, index as u32 + 4) {
                // let new_instr = InstructionBlock::Four(
                //     refset[index],
                //     refset[index + 1],
                //     refset[index + 2],
                //     refset[index + 3],
                // );

                // set.insert(index, new_instr);
                set.remove(index + 1);
                set.remove(index + 2);
                set.remove(index + 3);
                set.remove(index + 4);
            }
        }

        // Try and merge 8 tuples
        for (index, instr) in refset.iter().rev().skip(7).enumerate() {
            if self.range_self_permutes(&mut check, index as u32, index as u32 + 8) {}
        }

        // Try and merge 16 tuples
        for (index, instr) in refset.iter().rev().skip(15).enumerate() {
            if self.range_self_permutes(&mut check, index as u32, index as u32 + 16) {}
        }

        set
    }

    // Returns whether a range of elements "self permutes" on
    // themselves, I.e. the array subset that can be optimized
    // to a single SIMD instruction.
    fn range_self_permutes(&self, set: &mut HashSet<u32>, lower: u32, upper: u32) -> bool {
        set.clear();

        // If we indices that are out of bounds, clear out.
        if lower < 0 || upper > (self.values.len() - 1) as u32 {
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
