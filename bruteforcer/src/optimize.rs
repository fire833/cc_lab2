use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mask {
    mask: u16,
}

impl Mask {
    const fn new() -> Self {
        Self { mask: 0x0000 }
    }

    fn set_ith(&mut self, i: u8) {
        if i < 16 {
            let mask = (0b0000000000000001 as u16) << i;
            self.mask |= mask;
        }
    }

    const fn get_ith(&self, i: u8) -> bool {
        if i < 16 {
            let mask = (0b0000000000000001 as u16) << i;
            self.mask & mask == mask
        } else {
            false
        }
    }
}

#[test]
fn test_mask() {
    let mut m1 = Mask::new();
    m1.set_ith(3);
    m1.set_ith(0);
    m1.set_ith(9);
    assert_eq!(true, m1.get_ith(3));
    assert_eq!(true, m1.get_ith(0));
    assert_eq!(false, m1.get_ith(11));
    assert_eq!(false, m1.get_ith(15));
    assert_eq!(0b0000001000001001, m1.mask);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleInstruction {
    index: u32,
    value: u32,

    mask: Mask,
}

impl SingleInstruction {
    const fn new(index: u32, value: u32) -> Self {
        Self {
            index,
            value,
            mask: Mask::new(),
        }
    }
}

impl From<(u32, u32)> for SingleInstruction {
    fn from(value: (u32, u32)) -> Self {
        Self {
            index: value.0,
            value: value.1,
            mask: Mask::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FourInstruction {
    value1: SingleInstruction,
    value2: SingleInstruction,
    value3: SingleInstruction,
    value4: SingleInstruction,

    mask: Mask,
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
            mask: Mask::new(),
        }
    }

    fn new_from_instr(instrs: Vec<InstructionBlock>) -> Option<Self> {
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

    mask: Mask,
}

impl EightInstruction {
    const fn new(value1: FourInstruction, value2: FourInstruction) -> Self {
        Self {
            value1,
            value2,
            mask: Mask::new(),
        }
    }

    fn new_from_instr(instrs: Vec<InstructionBlock>) -> Option<EightInstruction> {
        let size = 0;
        None
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

    mask: Mask,
}

impl SixteenInstruction {
    const fn new(value1: EightInstruction, value2: EightInstruction) -> Self {
        Self {
            value1,
            value2,
            mask: Mask::new(),
        }
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

impl InstructionBlock {
    fn len(&self) -> usize {
        match self {
            InstructionBlock::Single(_) => 1,
            InstructionBlock::Four(_) => 2,
            InstructionBlock::Eight(_) => 8,
            InstructionBlock::Sixteen(_) => 16,
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

/// A shiftmask wrapper struct.
pub struct ShiftMask {
    /// The array of values of this permutation mask (ie our input).
    values: Vec<u32>,
}

impl ShiftMask {
    const fn new(values: Vec<u32>) -> Self {
        Self { values }
    }

    pub const SIMD_COUNTS: [u8; 3] = [4, 8, 16];

    fn optimize_to_blocks(&self) -> Vec<InstructionBlock> {
        // check through all 4 blocks
        let mut set1: Vec<InstructionBlock> = vec![];
        let mut set2: Vec<InstructionBlock> = vec![];

        let mut store_vec: Vec<SingleInstruction> = vec![];

        for (index, val) in self.values.iter().enumerate() {
            set1.push(InstructionBlock::Single(SingleInstruction::new(
                index as u32,
                *val,
            )));
        }

        let mut output: &Vec<InstructionBlock> = &vec![];

        for simd_count in Self::SIMD_COUNTS.iter() {
            let mut sum = 0;
            let mut queue: Vec<InstructionBlock> = vec![];

            while set1.len() > 0 {
                if let Some(instr) = set1.pop() {
                    sum += instr.len() as u8;
                    queue.push(instr);
                }

                // If we have reached a total sum, we need to
                // check for locality of the data.
                if sum == *simd_count {
                    // If we have a self permutation, then we need to merge
                    // these instructions in the queue to a higher order instruction.
                    if Self::chunk_self_permutes(&mut store_vec, &queue, *simd_count) {}
                }
            }

            set1 = set2;
            set2 = vec![];
            output = &set2;
        }

        output.to_vec()
    }

    // Returns whether a range of elements "self permutes" on
    // themselves, I.e. the array subset that can be optimized
    // to a single SIMD instruction.
    fn chunk_self_permutes(
        full_vec: &mut Vec<SingleInstruction>,
        chunk: &Vec<InstructionBlock>,
        simd_count: u8,
    ) -> bool {
        full_vec.clear();

        for blk in chunk.iter() {
            let mut cloned: Vec<SingleInstruction> = blk.clone().into();
            full_vec.append(&mut cloned);
        }

        let mut min_src: u32 = 0;
        let mut max_src: u32 = u32::MAX;
        let mut min_dst: u32 = 0;
        let mut max_dst: u32 = u32::MAX;

        for instr in full_vec.iter() {
            if instr.index > min_src {
                min_src = instr.index;
            }
            if instr.index < max_src {
                max_src = instr.index;
            }
            if instr.value > min_dst {
                min_dst = instr.value;
            }
            if instr.value < max_dst {
                max_dst = instr.value;
            }
        }

        (max_src - min_src == (simd_count - 1) as u32)
            && (max_dst - min_dst == (simd_count - 1) as u32)
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
