use std::collections::VecDeque;

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
}

impl EightInstruction {
    const fn new(value1: FourInstruction, value2: FourInstruction) -> Self {
        Self { value1, value2 }
    }

    fn new_from_instr(instrs: Vec<InstructionBlock>) -> Option<EightInstruction> {
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

    fn new_from_instr(instrs: Vec<InstructionBlock>) -> Option<SixteenInstruction> {
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
            InstructionBlock::Four(_) => 4,
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

impl From<Vec<u32>> for ShiftMask {
    fn from(value: Vec<u32>) -> Self {
        Self { values: value }
    }
}

impl ShiftMask {
    #[allow(unused)]
    const fn new(values: Vec<u32>) -> Self {
        Self { values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    // pub const SIMD_COUNTS: [u8; 3] = [4, 8, 16];
    pub const SIMD_COUNTS: [u8; 2] = [4, 8];

    pub fn permute_array_by_mask(&self, input: &Vec<u32>) -> Vec<u32> {
        let mut output = vec![];

        for _ in 0..input.len() {
            output.push(0);
        }

        for (index, value) in self.values.iter().enumerate() {
            output[*value as usize] = input[index];
        }

        output
    }

    pub fn optimize_to_blocks(&self) -> VecDeque<InstructionBlock> {
        // check through all 4 blocks
        let mut set1: VecDeque<InstructionBlock> = VecDeque::new();
        let mut set2: VecDeque<InstructionBlock> = VecDeque::new();

        let mut store_vec: Vec<SingleInstruction> = vec![];

        for (index, val) in self.values.iter().enumerate() {
            set1.push_back(InstructionBlock::Single(SingleInstruction::new(
                index as u32,
                *val,
            )));
        }

        for simd_count in Self::SIMD_COUNTS.iter() {
            let mut sum = 0;
            let mut queue: Vec<InstructionBlock> = vec![];

            while set1.len() > 0 {
                if let Some(instr) = set1.pop_front() {
                    sum += instr.len() as u8;
                    queue.push(instr);
                }

                // If we have reached a total sum, we need to
                // check for locality of the data.
                if sum == *simd_count {
                    // If we have a self permutation, then we need to merge
                    // these instructions in the queue to a higher order instruction.
                    if Self::chunk_self_permutes(&mut store_vec, &queue, *simd_count) {
                        match *simd_count {
                            4 => {
                                if let Some(four) = FourInstruction::new_from_instr(queue.clone()) {
                                    set2.push_back(InstructionBlock::Four(four));
                                    queue.clear();
                                    sum = 0;
                                }
                            }
                            8 => {
                                if let Some(eight) = EightInstruction::new_from_instr(queue.clone())
                                {
                                    set2.push_back(InstructionBlock::Eight(eight));
                                    queue.clear();
                                    sum = 0;
                                }
                            }
                            16 => {
                                if let Some(sixteen) =
                                    SixteenInstruction::new_from_instr(queue.clone())
                                {
                                    set2.push_back(InstructionBlock::Sixteen(sixteen));
                                    queue.clear();
                                    sum = 0;
                                }
                            }
                            _ => {}
                        }
                    } else {
                        let elem = queue.remove(0);
                        sum -= elem.len() as u8;
                        set2.push_back(elem);
                    }
                    // If we have overrun the sum, then we need to pop the front element
                    // from the queue and assign it to set 2.
                } else if sum > *simd_count {
                    let elem = queue.remove(0);
                    sum -= elem.len() as u8;
                    set2.push_back(elem);
                    // If we have no elements left in the set, then move the
                    // queue over to the output set.
                } else if set1.len() == 0 {
                    while queue.len() > 0 {
                        let elem = queue.remove(0);
                        set2.push_back(elem);
                    }
                } else if sum < *simd_count {
                    continue;
                }
            }

            set1 = set2;
            set2 = VecDeque::new();
        }

        set1
    }

    // Returns whether a range of elements "self permutes" on
    // themselves, I.e. the array subset that can be optimized
    // to a single SIMD instruction.
    pub fn chunk_self_permutes(
        full_vec: &mut Vec<SingleInstruction>,
        chunk: &Vec<InstructionBlock>,
        simd_count: u8,
    ) -> bool {
        full_vec.clear();

        for blk in chunk.iter() {
            let mut cloned: Vec<SingleInstruction> = blk.clone().into();
            full_vec.append(&mut cloned);
        }

        let mut min_src: i32 = i32::MAX;
        let mut max_src: i32 = i32::MIN;
        let mut min_dst: i32 = i32::MAX;
        let mut max_dst: i32 = i32::MIN;

        for instr in full_vec.iter() {
            if (instr.index as i32) < min_src {
                min_src = instr.index as i32;
            }
            if (instr.index as i32) > max_src {
                max_src = instr.index as i32;
            }
            if (instr.value as i32) < min_dst {
                min_dst = instr.value as i32;
            }
            if (instr.value as i32) > max_dst {
                max_dst = instr.value as i32;
            }
        }

        (max_src - min_src == (simd_count - 1) as i32)
            && (max_dst - min_dst == (simd_count - 1) as i32)
    }
}

#[test]
fn test_canonical_permute() {
    let mask = ShiftMask::new(vec![3, 2, 1, 0]);
    assert_eq!(
        vec![4, 3, 2, 1],
        mask.permute_array_by_mask(&vec![1, 2, 3, 4])
    )
}

#[test]
fn test_self_permute() {
    let mut tmp_vec: Vec<SingleInstruction> = vec![];

    assert_eq!(
        true,
        ShiftMask::chunk_self_permutes(
            &mut tmp_vec,
            &vec![
                InstructionBlock::Single(SingleInstruction::new(0, 30)),
                InstructionBlock::Single(SingleInstruction::new(1, 29)),
                InstructionBlock::Single(SingleInstruction::new(2, 28)),
                InstructionBlock::Single(SingleInstruction::new(3, 27)),
            ],
            4
        )
    );

    assert_eq!(
        true,
        ShiftMask::chunk_self_permutes(
            &mut tmp_vec,
            &vec![
                InstructionBlock::Single(SingleInstruction::new(4, 1)),
                InstructionBlock::Four(FourInstruction::new(
                    SingleInstruction::new(0, 0),
                    SingleInstruction::new(1, 7),
                    SingleInstruction::new(2, 6),
                    SingleInstruction::new(3, 4),
                )),
                InstructionBlock::Single(SingleInstruction::new(5, 3)),
                InstructionBlock::Single(SingleInstruction::new(6, 2)),
                InstructionBlock::Single(SingleInstruction::new(7, 5)),
            ],
            8
        )
    );
}

#[test]
fn test_dp() {
    let mask = ShiftMask::new(vec![1, 2, 3, 0, 4, 7, 5, 6, 8]);
    assert_eq!(
        mask.optimize_to_blocks(),
        vec![
            InstructionBlock::Eight(EightInstruction::new(
                FourInstruction::new(
                    SingleInstruction::new(0, 1),
                    SingleInstruction::new(1, 2),
                    SingleInstruction::new(2, 3),
                    SingleInstruction::new(3, 0),
                ),
                FourInstruction::new(
                    SingleInstruction::new(4, 4),
                    SingleInstruction::new(5, 7),
                    SingleInstruction::new(6, 5),
                    SingleInstruction::new(7, 6),
                ),
            )),
            InstructionBlock::Single(SingleInstruction::new(8, 8))
        ]
    );
}
