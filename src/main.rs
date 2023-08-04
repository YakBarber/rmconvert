// #![allow(dead_code, unused_imports)]

use std::fs::{read,write};

use nom::multi::many1;

use rmconvert::types::*;
use rmconvert::parse::*;
use rmconvert::svg::*;
use rmconvert::util::*;

use std::path::Path;


fn main() {

    let last = last_modified_page(Path::new("/home/barry/mnt/sshrm/xochitl/")).unwrap();
    dbg!(&last);

    let bytes: &[u8] = &read(&last).unwrap();

    let mut out = bytes.clone().to_vec();

    let (_input, blocks) = many1(parse_block)(&bytes[163..]).unwrap();

    let last_id = 
        match &blocks.last().unwrap() {
            Block::Line(l) => {
                l.line_id.clone()
            },
            _ => {
                IdField([0x00,0x00,0x00])
            },
        };

    for arg in std::env::args() {

        // recreate notebook from svg
        if arg=="1" {
            let lines = read_svg(last_id, "test.svg").unwrap();

            for line in lines.iter() {

                let mut raw_block = RawBytes::from(Block::Line(line.clone()));
                out.append(&mut raw_block);
            };

            write(last,out).unwrap();
            break;
        }

        // create svg from notebook
        else if arg=="2" {
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
            //paths.push(create_border_path());
            write_svg(paths, "test.svg").unwrap();
        };
    };





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

}


