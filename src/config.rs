
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use anyhow::Result;

use crate::cli::OutputFormat;
//use crate::types::Error;

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct MarkdownCfg { }

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct SvgCfg { }

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct DebugCfg { }

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct BytesCfg { }

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct JsonCfg { }

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct OutputCfg {
    pub default: OutputFormat,
    pub markdown: MarkdownCfg,
    pub svg: SvgCfg,
    pub debug: DebugCfg,
    pub bytes: BytesCfg,
    pub json: JsonCfg,
}


#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub output: OutputCfg,
    pub rm_path: PathBuf,
}


impl Settings {
    pub fn empty() -> Self {
        Settings{
            output: OutputCfg{
                default: OutputFormat::Markdown,
                markdown: MarkdownCfg {  },
                svg: SvgCfg {  },
                debug: DebugCfg {  },
                bytes: BytesCfg {  },
                json: JsonCfg {  },
            },
            rm_path: PathBuf::new(),
        }
    }


    pub fn new() -> Result<Self> {
        // default config first
        //dotenv::dotenv();
        //let s = config::Config::builder()
        //    .add_source(config::File::with_name("assets/default_config.toml"))
        //    // then user config
        //    //.add_source(config::File::with_name("todo"))
        //    // then environment
        //    .add_source(config::Environment::with_prefix("RMCONVERT"))

        //    .build()?;
        //Ok(s.try_deserialize()?)

        Ok(Settings::empty())
    }
}
