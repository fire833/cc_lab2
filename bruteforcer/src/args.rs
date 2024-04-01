use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author = "Kendall Tauser",
    version = "0.0.1",
    about = "Lab 2 Bruteforcer"
)]
pub struct BruteforcerArgs {
    /// Specify the subcommand you wish to run.
    #[command(subcommand)]
    pub cmd: BruteforcerCmds,

    /// Specify the pattern to generate output code for.
    #[arg(long, short, value_delimiter = ',')]
    pub pattern: Option<Vec<u32>>,

    /// Length of random pattern
    #[arg(short, long, default_value_t = 50)]
    pub len: u32,

    #[arg(long, short, default_value_t = String::from("amd64"))]
    pub arch: String,
}

#[derive(Subcommand, Debug)]
pub enum BruteforcerCmds {
    #[command(alias = "bf")]
    Bruteforce,

    /// Generate C function body for manual insertion into
    /// a C program for later compilation. Done with basic
    /// optimization of blocks, and print in original ordering.
    #[command(alias = "simplec")]
    SimpleCFunc,

    /// Generate a random pattern of specified length, and
    /// tries to create a pattern that contains subsets that can
    /// be SIMDized.
    #[command(alias = "randpat")]
    RandomPattern,
}
