use clap::{Args, Subcommand, Parser};

use clio::{Input, Output};


#[derive(Debug, Subcommand)]
pub enum Commands {
    Dump(DumpArgs),
}

#[derive(Debug, Args)]
pub struct DumpArgs {
    #[clap(value_parser)]
    input: Input,

    #[clap(value_parser)]
    output: Output,
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}
