
use std::path::PathBuf;

use clap::{Args, Subcommand, Parser, ValueEnum};
use serde::{Serialize, Deserialize};

use clio::{Input, Output};


#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Extract a single reMarkable page-file to SVG.
    Extract(ExtractArgs),

    /// Draw a new shape into a reMarkable file
    Draw(DrawArgs),
    
    /// Read a reMarkable file and structure and content info
    Stats(StatsArgs),
}

#[derive(Debug, Subcommand, Clone, ValueEnum, Serialize, Deserialize)]
pub enum OutputFormat {
    Markdown,
    JSON,
    SVG,
    Debug,
    Bytes,
}

#[derive(Debug, Args)]
pub struct StatsArgs {
    /// reMarkable file to read
    #[clap(value_parser)]
    #[arg(short, long, group = "inargs")]
    pub input: Option<Input>,
    
    /// Attempt to read from the last opened reMarkable page file (slow)
    #[clap(value_parser)]
    #[arg(short, long, group = "inargs")]
    pub last: bool,
}

#[derive(Debug, Args, Clone)]
pub struct ExtractArgs {

    /// reMarkable file to extract from.
    #[clap(value_parser)]
    #[arg(short, long, group = "inargs")]
    pub input: Option<Input>,

    /// SVG file to extract into. If none is given, dump SVG to STDOUT.
    #[clap(value_parser)]
    #[arg(short, long)]
    pub output: Option<Output>,
    
    /// Attempt to read from the last opened reMarkable page file (slow)
    #[clap(value_parser)]
    #[arg(short, long, group = "inargs")]
    pub last: bool,

    #[arg(short='t', long, default_value="debug")]
    pub format: OutputFormat,

    #[arg(short='x', long)]
    pub skip_text: bool,

    #[arg(short='X', long)]
    pub skip_lines: bool,
}

#[derive(Debug, Args)]
pub struct DrawArgs {
    /// reMarkable file to target.
    #[clap(value_parser)]
    #[arg(short,long, group="gtarget")]
    pub output: Option<Output>,
    
    /// Attempt to target the last opened reMarkable page file (slow). 
    /// Will fail if no file can be found.
    #[clap(value_parser)]
    #[arg(short,long, group="gtarget")]
    pub last: bool,

    #[clap(value_parser)]
    #[arg(long)]
    pub svg: Option<Input>,

    #[clap(value_parser)]
    #[arg(long)]
    pub svg_path: Option<String>,

    #[clap(value_parser)]
    #[arg(long)]
    pub width: Option<String>,

    #[clap(value_parser)]
    #[arg(long)]
    pub pen: Option<String>,

    #[clap(value_parser)]
    #[arg(long)]
    pub layer: Option<String>,

    #[clap(value_parser)]
    #[arg(long)]
    pub color: Option<String>,
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
    pub rm_path: Option<PathBuf>,
}

//impl Cli {
//
//    /// works like Parser::parse, but will fill in missing args that have equivalent
//    /// environment variables set. Args, when supplied, always supercede environment variables.
//    pub fn parse_with_env() -> Self {
//        // recursively load relevant .env files
//        dotenv::dotenv().ok();
//
//        let mut cui = Cli::parse();
//
//        cui.rm_path = cui.rm_path.or_else( || {
//            if envmnt::exists("RMCONVERT_RM_DIR") {
//                let dir: String = envmnt::get_parse("RMCONVERT_RM_DIR").unwrap();
//                Some(PathBuf::from(dir))
//            }
//            else {
//                None
//            }
//        });
//
//        cui
//    }
//}

