use args::BruteforcerCmds;
use clap::Parser;
use instructions_x86_64::{Operand, Register};
use itertools::Itertools;
use playground::Playground;

use crate::{
    encodings::{CEncoder, SerializeAMD64MachineCode},
    instructions_x86_64::Instruction,
    optimize::ShiftMask,
};

mod abstract_instructions;
mod args;
mod encodings;
mod instructions_x86_64;
mod optimize;
mod playground;

fn main() {
    let args = args::BruteforcerArgs::parse();

    match args.cmd {
        BruteforcerCmds::Bruteforce => {
            if let Some(mask) = args.pattern {
                let mask: ShiftMask = mask.into();
                let blocks = mask.optimize_to_blocks();
                let pg: Playground;
                unsafe {
                    pg = Playground::new(4096);
                }

                let mut program_buffer: Vec<u8> = vec![];

                for blks in blocks.iter().permutations(blocks.len()) {
                    Instruction::RDTSC.write_amd64_bytes(&mut program_buffer);
                    for blk in blks.iter() {
                        blk.write_amd64_bytes(&mut program_buffer);
                    }
                    Instruction::RDTSC.write_amd64_bytes(&mut program_buffer);
                    Instruction::RET.write_amd64_bytes(&mut program_buffer);
                    let _output = pg.run_is_correct(&program_buffer, &mask);
                }
            } else {
                println!("please provide a pattern to bruteforce possible solutions")
            }
        }
        BruteforcerCmds::SimpleCFunc => {
            if let Some(mask) = args.pattern {
                let mask: ShiftMask = mask.into();
                let blocks = mask.optimize_to_blocks();

                for (i, block) in blocks.iter().enumerate() {
                    print!("{}", block.encode_to_c(i as u32));
                }
            } else {
                println!("please provide a pattern to generate a corresponding C function body")
            }
        }
    }
}
