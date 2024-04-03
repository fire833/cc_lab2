use std::{collections::VecDeque, fmt::Display};

use rand::Rng;
use rand::{seq::SliceRandom, thread_rng};

use crate::abstract_instructions::{
    eight::EightInstruction, four::FourInstruction, single::SingleInstruction,
    sixteen::SixteenInstruction, InstructionBlock,
};

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

impl Display for ShiftMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, val) in self.values.iter().enumerate() {
            if i < self.values.len() - 1 {
                if let Err(e) = write!(f, "{},", *val) {
                    return Err(e);
                }
            } else {
                if let Err(e) = write!(f, "{}", *val) {
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}

impl ShiftMask {
    #[allow(unused)]
    const fn new(values: Vec<u32>) -> Self {
        Self { values }
    }

    pub fn new_random(len: u32) -> Self {
        let mut dst_set: Vec<u32> = vec![];
        let mut src_set: Vec<u32> = (0 as u32..len as u32).collect();

        while src_set.len() > 0 {
            let len = src_set.len();

            match rand::thread_rng().gen_range(1..=4) {
                1 => dst_set.push(src_set.remove(rand::thread_rng().gen_range(0..=len - 1))),
                2 => {
                    let base_index = rand::thread_rng().gen_range(0..=len - 1);
                    if len >= 4 && len - base_index >= 4 {
                        let mut loc_vec = vec![];

                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));

                        loc_vec.shuffle(&mut thread_rng());
                        dst_set.append(&mut loc_vec);
                    }
                }

                3 => {
                    let base_index = rand::thread_rng().gen_range(0..=len - 1);
                    if len >= 8 && len - base_index >= 8 {
                        let mut loc_vec = vec![];

                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));

                        loc_vec.shuffle(&mut thread_rng());
                        dst_set.append(&mut loc_vec);
                    }
                }

                4 => {
                    let base_index = rand::thread_rng().gen_range(0..=len - 1);
                    if len >= 16 && len - base_index >= 16 {
                        let mut loc_vec = vec![];

                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));
                        loc_vec.push(src_set.remove(base_index));

                        loc_vec.shuffle(&mut thread_rng());
                        dst_set.append(&mut loc_vec);
                    }
                }
                _ => continue,
            }
        }

        Self { values: dst_set }
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

    pub fn optimize_to_blocks(&self, num_iter: u8) -> VecDeque<InstructionBlock> {
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
            if *simd_count > num_iter {
                continue;
            }

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
                } else if sum < *simd_count {
                    continue;
                }
            }

            while queue.len() > 0 {
                let elem = queue.remove(0);
                set2.push_back(elem);
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
        mask.optimize_to_blocks(255),
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
