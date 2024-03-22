use args::BruteforcerCmds;
use clap::Parser;
use playground::Playground;

use crate::optimize::ShiftMask;

mod args;
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
                println!("running program now");

                let prog = vec![0xc3];
                let output = pg.run_is_correct(&prog, &mask);
                println!("{:?}", output);
            } else {
                println!("please provide a pattern to bruteforce possible solutions")
            }
        }
        BruteforcerCmds::SimpleCFunc => {
            if let Some(mask) = args.pattern {
                let mask: ShiftMask = mask.into();
                let blocks = mask.optimize_to_blocks();
            } else {
                println!("please provide a pattern to generate a corresponding C function body")
            }
        }
    }
}
