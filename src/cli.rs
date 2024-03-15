
use std::path::PathBuf;

use clap::{Args, Subcommand, Parser, ValueEnum};
use serde::{Serialize, Deserialize};

use clio::{Input, Output};


#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Extract a single reMarkable page-file to SVG.
    Extract(ExtractArgs),

    /// Create a new reMarkable file and add line data to it
    Create(CreateArgs),
    
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

    /// Draw the contents of an SVG file.
    #[clap(value_parser)]
    #[arg(long)]
    pub from_file: Option<Input>,
    
    /// Create a new path. Requires at least 1 of <descriptor args>
    #[clap(value_parser)]
    #[arg(long)]
    pub path: bool,
    
    /// Draw the shape with its raw SVG args.
    ///
    /// TODO: target these first
    /// M x  y
    /// m dx dy
    /// L x  y
    /// l dx dy
    /// H x
    /// h dx
    /// V    y
    /// v    dy
    ///
    #[clap(value_parser)]
    #[arg(long)]
    pub raw: Option<String>,

    #[clap(value_parser)]
    #[arg(long, group="gpath")]
    pub stroke: Option<u8>,

    //// other shapes here. focus on path first
    //
    // /// Create a new line. Requires at least 1 of <descriptor args>
    // #[clap(value_parser)]
    // #[arg(long, group="line")]
    // pub line: bool,

    // /// Create a new circle. Requires at least 1 of <descriptor args>
    // #[clap(value_parser)]
    // #[arg(long, group="circle")]
    // pub circle: bool,
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

