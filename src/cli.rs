
use std::path::PathBuf;

use clap::{Args, Subcommand, Parser, ValueEnum};

use clio::{Input, Output};


#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Extract a single reMarkable page file to SVG.
    Extract(ExtractArgs),

    /// Create a new reMarkable file and add line data to it
    Create(CreateArgs),
    
    /// Insert line data into an *existing* reMarkable file
    Insert(InsertArgs),
}

#[derive(Debug, Subcommand, Clone, ValueEnum)]
pub enum OutputFormat {
    Human,
    JSON,
    SVG,
    Raw,
    Bytes,
}

#[derive(Debug, Args)]
pub struct ExtractArgs {

    /// reMarkable file to extract from.
    #[clap(value_parser)]
    #[arg(short,long, group = "inargs")]
    pub input: Option<Input>,

    /// SVG file to extract into. If none is given, dump SVG to STDOUT.
    #[clap(value_parser)]
    #[arg(short,long)]
    pub output: Option<Output>,
    
    /// Attempt to read from the last opened reMarkable page file (slow)
    #[clap(value_parser)]
    #[arg(short,long, group = "inargs")]
    pub last: bool,

    #[arg(short='t', long, default_value="human")]
    pub format: OutputFormat,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// SVG file to create from.
    #[clap(value_parser)]
    #[arg(short,long)]
    pub input: Option<Input>,

    /// reMarkable file to create. Will not overwrite unless --force is given.
    #[clap(value_parser)]
    #[arg(short,long, group = "inargs")]
    pub output: Option<Output>,
    
    /// Attempt to replace the last opened reMarkable page file (slow). 
    /// Requires the --force flag, otherwise it will fail.
    #[clap(value_parser)]
    #[arg(short,long, group = "inargs")]
    pub last: bool,

    /// Force an overwrite operation if the file to be created already exists.
    #[clap(value_parser)]
    #[arg(short,long)]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct InsertArgs {
    /// SVG file to create from.
    #[clap(value_parser)]
    #[arg(short,long, group = "inargs")]
    pub input: Option<Input>,

    /// reMarkable file to insert into
    #[clap(value_parser)]
    #[arg(short,long, group = "inargs")]
    pub output: Output,
    
    /// Attempt to replace the last opened reMarkable page file (slow)
    #[clap(value_parser)]
    #[arg(short,long, group = "inargs")]
    pub last: bool,

    /// layer in the reMarkable file to use (?)
    #[clap(value_parser)]
    #[arg(long)]
    pub layer: Option<u8>,

}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// A directory containing reMarkable notebook files to search in as an additional working
    /// directory. 
    ///
    /// Identical to setting the environment variable `RMCONVERT_RM_DIR`. 
    ///
    /// In the tablet itself, this directory is `/home/root/.local/share/remarkable/xochitl`.
    #[arg(short, long, name = "RM_DIR")]
    pub remarkable_dir: Option<PathBuf>,
}

impl Cli {
    /// works like Parser::parse, but will fill in missing args that have equivalent
    /// environment variables set. Args, when supplied, always supercede environment variables.
    pub fn parse_with_env() -> Self {
        // recursively load relevant .env files
        dotenv::dotenv().ok();

        let mut cui = Cli::parse();

        cui.remarkable_dir = cui.remarkable_dir.or_else( || {
            if envmnt::exists("RMCONVERT_RM_DIR") {
                let dir: String = envmnt::get_parse("RMCONVERT_RM_DIR").unwrap();
                Some(PathBuf::from(dir))
            }
            else {
                None
            }
        });

        cui
    }
}

