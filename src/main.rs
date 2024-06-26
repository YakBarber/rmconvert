#![allow(dead_code, unused_variables, unused_imports)]


use std::io::{Write, Read, BufRead};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;

use rmconvert::types::*;
use rmconvert::parse::*;
use rmconvert::svg::*;
use rmconvert::util::*;
use rmconvert::cli::*;
use rmconvert::config::*;

use svg::node::element::Path;

use clap::Parser;
use clio::Input;

use log::warn;

fn extract_from_blocks(ExtractArgs {skip_text, skip_lines, .. }: ExtractArgs, notebook: Notebook) -> (Option<Vec<Path>>, Option<Vec<String>>) {

    (None, None)
}

/// Convert Blocks into SVG paths, and return the Vec<Path>.
///
/// Path creation is very simplistic, and not all Block types are supported.
fn blocks_to_svg_paths(notebook: Notebook) -> Vec<Path> {
    let mut paths = Vec::new();

    let blocks = notebook.blocks;

    for block in &blocks {
        if let Block::Line(line) = block {
            let path = path_from_line(&line);
            match path {
                None => {},
                Some(p) => {
                    paths.push(p);
                },
            };
        };
    };

    warn!("SVG path extraction ignores text");
    paths
}

fn file_to_blocks<R: Read>(mut rmpath: R) -> Result<Notebook> {
    let mut bytes: Vec<u8> = Vec::new();
    rmpath.read_to_end(&mut bytes)?;
    parse_full(&bytes)
}

fn write_blocks_to_rm_file(notebook: Notebook, file: PathBuf) -> Result<PathBuf> {
    todo!()
}

fn write_blocks<W: Write>(notebook: Notebook, writer: W) -> Result<()> {
    todo!()
}

/// Render the Notebook as a String, based on the required output format. 
///
/// All output formats are returned as Strings, until this abstraction proves problematic.
fn render(notebook: Notebook, format: OutputFormat, settings: Settings) -> Result<String> {
    match format {
        OutputFormat::Markdown => {
            render_markdown(notebook, settings.output.markdown)
        },
        OutputFormat::JSON => {
            render_json(notebook, settings.output.json)
        },
        OutputFormat::SVG => {
            render_svg(notebook, settings.output.svg)
        },
        OutputFormat::Debug => {
            render_debug(notebook, settings.output.debug)
        },
        OutputFormat::Bytes => {
            render_bytes(notebook, settings.output.bytes)
        },
    }
}

fn render_markdown(notebook: Notebook, cfg: MarkdownCfg) -> Result<String> {
    let mut strings = Vec::new();

    let blocks = notebook.blocks;

    for block in blocks {
        if let Block::TextDef(tdef) = block {
            for chunk in &tdef.texts {
                strings.push(chunk.text.clone());
            };
        };
    };

    warn!("Text rendering ignores text formatting");
    Ok(strings.join("\n"))
}

fn render_json(notebook: Notebook, cfg: JsonCfg) -> Result<String> {
    let json = serde_json::to_string(&notebook.blocks)?;
    Ok(json)
}

fn render_svg(notebook: Notebook, cfg: SvgCfg) -> Result<String> {
    todo!()
}

fn render_debug(notebook: Notebook, cfg: DebugCfg) -> Result<String> {
    Ok(format!("{:?}", notebook.blocks))
}

fn render_bytes(notebook: Notebook, cfg: BytesCfg) -> Result<String> {
    todo!()
}


// TODO: use stdin? Does it even make sense here?
// TODO: make the panics reprint the --help text
fn do_extract(eargs: ExtractArgs, rmdir: Option<PathBuf>) -> Result<Notebook> {

    let ExtractArgs {input, output, last, skip_lines, skip_text, border} = eargs;

    let notebook = match (input, last) {

        // no input or last-modified flag; panic
        (None, false) => {
            panic!("no input file, dunno what to do");
        },
        // use last modified rM file
        (None, true) => {
            if let Some(dir) = rmdir {
                let lastf = last_modified_page(&dir).unwrap();
                let cliopath = Input::new(&lastf).unwrap();
                file_to_blocks(cliopath)?
            } else {
                panic!("no rmdir to use!");
            }
        },
        // use input, ignore last flag with msg
        (Some(inp), true) => {
            eprintln!("Both --input and --last were given; ignoring --last...");
            file_to_blocks(inp)?
        },
        // use input
        (Some(inp), false) => {
            file_to_blocks(inp)?
        },
    };

    let Notebook{frontmatter, blocks} = notebook.clone();


    if !skip_lines {
        let svg_paths = blocks_to_svg_paths(notebook.clone());

        if let Some(out) = output.clone() {
            //svg_paths.push(create_border_path());
            write_svg(svg_paths, out.to_string()).unwrap();
        } else {
            write_svg_to_stdout(svg_paths).unwrap();
        };
    };

    if !skip_text {
        let text = render_markdown(notebook.clone(), MarkdownCfg{});

        if let Some(_out) = output.clone() {
            todo!("Can't write text to files yet");
        } else {
            warn!("assuming ASCII...");

            for line in &text {
                let bytes = line.clone().into_bytes().into_boxed_slice();
                std::io::stdout().write_all(&bytes).unwrap();
            };
        };
    };

    Ok(Notebook{frontmatter, blocks})
}

#[allow(unused_variables, unused_mut)]
fn file_to_stats<R: Read>(mut rmpath: R) -> Result<()> {
    
    let Notebook{frontmatter: fm, blocks} = file_to_blocks(rmpath)?;

    let mut num_blocks = blocks.len();
    let mut num_lines = 0;
    let mut num_points = 0;
    let mut num_layer_defs = 0;
    let mut num_layer_names = 0;
    let mut num_layer_infos = 0;
    let mut num_text = 0;
    let mut num_text_chunks = 0;
    let mut num_text_backs = 0;
    let mut all_text: Vec<TextDef> = Vec::new();

    for block in &blocks {
        num_blocks = num_blocks + 1;

        match block {
            Block::Line(line) => {
                num_lines = num_lines + 1;
                num_points = num_points + line.points.len();
            },
            Block::LayerDef(layer) => {
                num_layer_defs = num_layer_defs + 1;
            },
            Block::TextDef(text) => {
                num_text = num_text + 1;
                num_text_chunks = num_text_chunks + text.texts.len();
                num_text_backs = num_text_backs + text.backmatter.len();
                all_text.push(text.clone());
            },
            Block::LayerName(name) => {
                num_layer_names = num_layer_names + 1;
            },
            Block::LayerInfo(info) => {
                num_layer_infos = num_layer_infos + 1;
            },
            Block::Unknown(flag, raw) => {
            },
        };
    };

    // ultimately this needs to be more flexible/useful obviously
    println!("Version: {:?}", fm.version);
    println!("Total Blocks: {}", num_blocks);
    println!("Total Lines: {}", num_lines);
    println!("Total Points in those lines: {}", num_points);
    println!("Total Layer Defs: {}", num_layer_defs);
    println!("Total Layer Names: {}", num_layer_names);
    println!("Total Layer Infos: {}", num_layer_infos);
    println!("Total Text Objects: {}", num_text);
    println!("Total Text Chunks: {}", num_text_chunks);
    println!("Total Text Backmatter: {}", num_text_chunks);
    println!("The Texts: {:?}", all_text);

    Ok(())
}

fn main() -> Result<()> {

    env_logger::init();

    let cli = Cli::parse();

    let settings = Settings::new()?;

    match cli.command {
        Commands::Extract(e_args) => {
            let notebook = do_extract(e_args.clone(), cli.rm_path)?;
            //let out_str = render(notebook, e_args.format, settings);
        },
        Commands::Draw(d_args) => {

            let e_args = ExtractArgs {
                input: match d_args.target.output {
                    None => None,
                    Some(p) => Some(Input::try_from(p.path().clone())?),
                },
                output: None,
                last: d_args.target.last,
                skip_text: false,
                skip_lines: false,
                border: false,
            };
            let mut notebook = do_extract(e_args, cli.rm_path)?;

            // file/stdin, then path. text is separate but comes last
            if let Some(mut svg) = d_args.input.svg {
                // load svg file into Blocks and add to notebook
                let mut raw = String::new();
                svg.lock().read_to_string(&mut raw)?;
                let lines = read_svg_buffer(&raw[..])?;
                let mut blocks = lines.into_iter().map(|l| Block::Line(l)).collect();
                notebook.blocks.append(&mut blocks);
            };
            if let Some(path) = d_args.input.path {
                // stick in a Data and do a ::from, then add to notebook
                Err(RMError::NotImplementedError)?
            };
            if let Some(text) = d_args.input.text {
                // ???
                Err(RMError::NotImplementedError)?
            };

            //write notebook back to file
            write_blocks_to_rm_file(notebook, PathBuf::from_str("/home/barry/whatever.rm")?)?;


        },
        //Commands::Stats(s_args) => {
        //    if s_args.last {
        //        let lastf = last_modified_page(&ui.rm_path.unwrap()).unwrap();
        //        let cliopath = Input::new(&lastf).unwrap();
        //        file_to_stats(cliopath)?;
        //    }
        //    else {
        //        match &s_args.input {
        //            Some(file) => {
        //                file_to_stats(file.clone())?;
        //            },
        //            None => {
        //            },
        //        };
        //    };
        //    file_to_stats(s_args.input.unwrap())?;

        //},
    };

    Ok(())
}




