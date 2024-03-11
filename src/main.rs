#![allow(dead_code, unused_variables)]

use std::io::{Write, Read, stdin, stdout};
use std::path::PathBuf;

use anyhow::Result;

use rmconvert::types::*;
use rmconvert::parse::*;
use rmconvert::svg::*;
use rmconvert::util::*;
use rmconvert::cli::*;

use svg::node::element::Path;

use clio::Input;

use log::warn;

fn extract_from_blocks(ExtractArgs {skip_text, skip_lines, .. }: ExtractArgs, blocks: Vec<Block>) -> (Option<Vec<Path>>, Option<Vec<String>>) {

    (None, None)
}

/// Convert Blocks into SVG paths, and return the Vec<Path>.
///
/// Path creation is very simplistic, and not all Block types are supported.
fn blocks_to_svg_paths(blocks: Vec<Block>) -> Vec<Path> {
    let mut paths = Vec::new();

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

fn render_markdown(notebook: Notebook, cfg: MarkdownCfg) -> Result<String> {
    let mut strings = Vec::new();

    for block in &notebook.blocks {
        if let Block::TextDef(tdef) = block {
            for chunk in &tdef.texts {
                strings.push(chunk.text.clone());
            };
        };
    };

    warn!("Text rendering ignores text formatting");
    Ok(strings.join("\n"))
}

// TODO: use stdin? Does it even make sense here?
// TODO: Return a Result<()>?
// TODO: make the panics reprint the --help text
fn do_extract(eargs: ExtractArgs, rmdir: Option<PathBuf>) -> Result<Notebook> {

    let ExtractArgs {input, output, last, format: _f, skip_lines, skip_text} = eargs;

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
        let svg_paths = blocks_to_svg_paths(blocks.clone());

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

// TODO: need to generate frontmatter, and other files? how to do this?
// TODO: Actually as it is now this is pretty much just do_insert. Change it?
// TODO: Create an example output file to use.
// TODO: This is a godddamn mess
fn do_create(cargs: CreateArgs, rmdir: Option<PathBuf>) -> Result<Notebook> {

    let CreateArgs {input, output, last, force} = cargs;

    let in_svg: Vec<Line> = {
        // given via argument, use this one. Ignore stdin.
        match input {
            Some(inp) => {
                // read from inp
                read_svg_file(inp.path().to_path_buf()).unwrap()
            },
            None => {
            // try to read from stdin
            let mut raw = String::new();
            stdin().read_to_string(&mut raw).unwrap();

            // try to parse
            read_svg_buffer(&raw[..]).unwrap()
            },
        }
    };

    let lines = match (output, last, force) {
        
        // no output or last-modified flag; panic
        (None, false, _) => {
            Err(Error::ArgsError("No output file given.".to_string()))

        },
        
        // last-modified flag, but without force
        (None, true, false) => {
            Err(Error::ArgsError("Can't overwrite without --force.".to_string()))
        },

        // last-modified flag, with force!
        (None, true, true) => {
            if let Some(dir) = rmdir {
                //let lastf = last_modified_page(&dir).unwrap();
                //write_blocks_to_rm_file(blocks, lastf);
                //Ok(blocks)
                todo!()
                
            } else {
                Err(Error::ArgsError("--last given, but no RM_DIR defined.".to_string()))
            }
        },

        // output file given, so write to it
        (Some( out), l, force) => {
            if l {
                eprintln!("Both --output and --last were given; ignoring --last...");
            };

            // if out is an actual filepath and not stdout
            if out.is_local() {
                let is_real = out.path().exists();
               
                // save file, with "permission" if it already exists
                if (is_real && force) || !is_real {
                    let mut out_blocks = Vec::new();

                    for line in in_svg.iter() {

                        let mut raw_block = RawBytes::from(Block::Line(line.clone()));
                        out_blocks.append(&mut raw_block);
                    };

                    Ok(out_blocks)
                }

                // need force and don't have it! D:
                else {
                    Err(Error::ArgsError("File exists. Use --force to overwrite".to_string()))
                }
            }
            
            // if the "file" is stdout
            else if out.is_std() {
                //just dump to stdout
                let mut out_blocks = Vec::new();

                for line in in_svg.iter() {

                    let mut raw_block = RawBytes::from(Block::Line(line.clone()));
                    stdout().write_all(&mut raw_block).unwrap();
                    out_blocks.append(&mut raw_block);
                };
                Ok(out_blocks)
            }
            else {
                panic!("Output isn't local file or std*");
            }
        },
    };

    //Ok((frontmatter, blocks))
    todo!()
}

#[allow(unused_variables)]
fn do_insert(iargs: InsertArgs, rmdir: Option<PathBuf>) {
    
    let InsertArgs {input, output, last, layer} = iargs;
    todo!();
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

    // TODO: remove this method entirely, replace with Settings::new()
    let ui = Cli::parse();
    let settings = Settings::new()?;

    match ui.command {
        Commands::Create(c_args) => {
            do_create(c_args, ui.rm_path)?;
        },
        Commands::Extract(e_args) => {
            do_extract(e_args, ui.remarkable_dir);
        },
        Commands::Insert(i_args) => {
            do_insert(i_args, ui.remarkable_dir);

        },
        Commands::Stats(s_args) => {
            if s_args.last {
                let lastf = last_modified_page(&ui.remarkable_dir.unwrap()).unwrap();
                let cliopath = Input::new(&lastf).unwrap();
                file_to_stats(cliopath);
            }
            else {
                match &s_args.input {
                    Some(file) => {
                        file_to_stats(file.clone());
                    },
                    None => {
                    },
                };
            };
            file_to_stats(s_args.input.unwrap());

        },
    };
}


