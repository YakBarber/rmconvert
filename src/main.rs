#![allow(dead_code, unused_imports)]

use std::fs::{read,write};
use std::io::{self, Write, Read, stdin, stdout};
use std::path::PathBuf;

use nom::multi::many1;

use rmconvert::types::*;
use rmconvert::parse::*;
use rmconvert::svg::*;
use rmconvert::util::*;
use rmconvert::cli::*;

use svg::node::element::Path;

use clap::Parser;
use clio::{Input, Output};


/// Convert a reMarkable file into Blocks, then into SVG paths, and return the Vec<Path>.
///
/// Path creation is very simplistic, and not all Block types are supported.
fn file_to_svg_paths<R: Read>(mut rmpath: R) -> Vec<Path> {
    //let bytes: &[u8] = &read(&rmpath).unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    rmpath.read_to_end(&mut bytes).unwrap();
    let (_input, blocks) = many1(parse_block)(&bytes[163..]).unwrap();

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
    paths
}


// TODO: use stdin? Does it even make sense here?
// TODO: Return a Result<()>?
fn do_extract(ExtractArgs { input, output, last }: ExtractArgs, rmdir: Option<PathBuf>) {

    let svg_paths: Vec<Path> = match (input, last) {

        // no input or last-modified flag; panic
        (None, false) => {
            panic!("no input file, dunno what to do");
        },
        // use last modified rM file
        (None, true) => {
            if let Some(dir) = rmdir {
                let lastf = last_modified_page(&dir).unwrap();
                let cliopath = Input::new(&lastf).unwrap();
                file_to_svg_paths(cliopath)
            } else {
                panic!("no rmdir to use!");
            }
        },
        // use input, ignore last flag with msg
        (Some(inp), true) => {
            eprintln!("Both --input and --last were given; ignoring --last...");
            file_to_svg_paths(inp)
        },
        // use input
        (Some(inp), false) => {
            file_to_svg_paths(inp)
        },
    };

    if let Some(out) = output {
        //svg_paths.push(create_border_path());
        write_svg(svg_paths, out.to_string()).unwrap();
    } else {
        write_svg_to_stdout(svg_paths).unwrap();
    };
}

// TODO: Return a Result<()>?
fn do_create(CreateArgs { input, output, last, force }: CreateArgs, rmdir: Option<PathBuf>) {
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

    match (output, last, force) {
        // no output or last-modified flag; panic
        (None, false, _) => {
            panic!("no output file given, dunno what to do");
        },
        // last-modified flag, but without force
        (None, true, false) => {
            panic!("Can't overwrite without --force.");
        },
        // last-modified flag, with force!
        (None, true, true) => {
            if let Some(dir) = rmdir {
                last_modified_page(&dir).unwrap()
            } else {
                panic!("no rmdir to use!");
            };
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
                }

                // need force and don't have it! D:
                else {
                    panic!("File exists. Use --force to overwrite");
                };
            }
            
            // if the "file" is stdout
            else if out.is_std() {
                //just dump to stdout
                for line in in_svg.iter() {

                    let mut raw_block = RawBytes::from(Block::Line(line.clone()));
                    stdout().write_all(&mut raw_block).unwrap();
                };
            };
        },
    };
}

fn main() {

    let ui = Cli::parse_with_env();

    match ui.command {
        Commands::Create(c_args) => {
            do_create(c_args, ui.remarkable_dir);
        },
        Commands::Extract(e_args) => {
            do_extract(e_args, ui.remarkable_dir);
        },
        Commands::Insert(_iargs) => {

        },
    };
}


    //let last = last_modified_page(Path::new("/home/barry/mnt/sshrm/xochitl/")).unwrap();
    //dbg!(&last);

    //let bytes: &[u8] = &read(&last).unwrap();

    //let mut out = bytes.clone().to_vec();

    //let (_input, blocks) = many1(parse_block)(&bytes[163..]).unwrap();

    //let last_id = 
    //    match &blocks.last().unwrap() {
    //        Block::Line(l) => {
    //            l.line_id.clone()
    //        },
    //        _ => {
    //            IdField([0x00,0x00,0x00])
    //        },
    //    };

    //for arg in std::env::args() {

    //    // recreate notebook from svg
    //    if arg=="1" {
    //        let lines = read_svg(last_id, "test.svg").unwrap();

    //        for line in lines.iter() {

    //            let mut raw_block = RawBytes::from(Block::Line(line.clone()));
    //            out.append(&mut raw_block);
    //        };

    //        write(last,out).unwrap();
    //        break;
    //    }

    //    // create svg from notebook
    //    else if arg=="2" {
    //        let mut paths = Vec::new();

    //        for block in &blocks {
    //            if let Block::Line(line) = block {
    //                let path = path_from_line(&line);
    //                match path {
    //                    None => {},
    //                    Some(p) => {
    //                        paths.push(p);
    //                    },
    //                };
    //            };
    //        };
    //        //paths.push(create_border_path());
    //        write_svg(paths, "test.svg").unwrap();
    //    };
    //};





    //let base_point = Point {
    //    x: -350.0,
    //    y: 320.0,
    //    speed: 1,
    //    width: 16,
    //    direction: 0,
    //    pressure: 22,
    //};

    //let mut points: Vec<Point> = Vec::new();
    //for i in 0..50 {
    //    let mut new = base_point.clone();
    //    new.y = new.y + f32::from(i*3 as u16);
    //    points.push(new);
    //};

    //let line = Line {
    //   layer_id: IdField([0x00, 0x0b, 0x00]), 
    //   line_id: new_id,
    //   last_line_id: last_id,
    //   id_field_0: IdField([0x00,0x00,0x00]), 
    //   pen_type: Some(17),
    //   color: Some(0),
    //   brush_size: Some(2.0),
    //   points,
    //};

    //let mut raw_block = RawBytes::from(Block::Line(line));

    //out.append(&mut raw_block);

    //write(last,out);
    
    // -------=

    ////blocks.iter().for_each(|b| println!("{:#?}",b));
    ////dbg!(blocks.len());

    //let mut paths = Vec::new();

    //for block in blocks {
    //    if let Block::Line(line) = block {
    //        let path = path_from_line(&line);
    //        match path {
    //            None => {},
    //            Some(p) => {
    //                paths.push(p);
    //            },
    //        };
    //    };
    //};

    //paths.push(create_border_path());


    //write_svg(paths, "test.svg").unwrap();


