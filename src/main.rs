#![allow(dead_code, unused_imports)]

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;


use std::fs::read;
use std::str::Utf8Error;
use std::vec::Vec;

use nom::{IResult, HexDisplay};
use nom::bytes::complete as bytes;
use nom::character::complete as char;
use nom::number::complete as num;
use nom::sequence::preceded;
use nom::multi::{length_value, length_data, length_count, many1, many0, count};
use nom::combinator::map as pmap;

const TEST_FILE_01: &str = "assets/213001cb-42c0-4628-8ed0-8320c15da2a8/110b4d92-e42e-4b78-a0cb-ebd40862f2f0.rm";

type IdField = [u8;2];


#[derive(Debug)]
struct Line {
   layer_id: IdField, 
   line_id: IdField, 
   last_line_id: IdField, 
   id_field_0: IdField, 
   pen_type: u32, 
   color: u32, 
   brush_size: f32, 
   points: Vec<Point>,
}

#[derive(Debug, Clone)]
struct Point {
    x: f32,
    y: f32,
    speed: u8,
    width: u8,
    direction: u8,
    pressure: u8,
}

#[derive(Debug)]
struct LayerDef {
    layer_id: IdField,
    unknown_1: Vec<u8>, //4 bytes
    unknown_2: Vec<u8>, //?? bytes
}

#[derive(Debug)]
struct LayerName {
    layer_id: IdField,
    id_field_0: IdField,
    name: String,
    unknown_rest: Vec<u8>,
}

#[derive(Debug)]
struct LayerInfo {
    id_field_0: IdField,
    id_field_1: IdField,
    id_field_2: IdField,
    id_field_3: IdField,
    layer_id: Option<IdField>
}

#[derive(Debug)]
struct TextDef {
    field_0: IdField,
    texts: Vec<TextChunk>,
    backmatter: Vec<TextBackmatter>,
    unknown_sized: String,
    unknown_unsized: String,
}

#[derive(Debug)]
struct TextChunk {
    chunk_id: [u8;3],  //sometimes it's 3
    other_chunk_id_0: [u8;3], //sometimes it's 3
    other_chunk_id_1: [u8;3], //sometimes it's 3
    done_flag: u32,
    text: String,
    magic_dollar: Option<u32>,
}

#[derive(Debug)]
struct TextBackmatter {
    field_0: IdField,
    field_1: IdField,
    field_2: IdField,
}

fn parse_text_chunk(input: &[u8]) -> IResult<&[u8], TextChunk> {
    let (input, len) = preceded(bytes::tag(&[0x0c]), num::le_u32)(input)?;
    let (final_input, rest) = bytes::take(len)(input)?;

    let (rest, chunk_id) = preceded(
                               bytes::tag(&[0x2f]), 
                               bytes::take_till(|b| b==0x3f),
                           )(rest)?;

    let (rest, other_chunk_id_0) = preceded(
                                       bytes::tag(&[0x3f]), 
                                       bytes::take_till(|b| b==0x4f),
                                   )(rest)?;

    let (rest, other_chunk_id_1) = preceded(
                                       bytes::tag(&[0x4f]), 
                                       bytes::take_till(|b| b==0x54),
                                   )(rest)?;

    let (rest, done_flag) = preceded(bytes::tag(&[0x54]), num::le_u32)(rest)?;

    let mut chunk = TextChunk{
        chunk_id: {
            let mut arr = [0;3];
            arr.copy_from_slice(&chunk_id);
            arr
        },
        other_chunk_id_0: {
            let mut arr = [0;3];
            arr.copy_from_slice(&other_chunk_id_0);
            arr
        },
        other_chunk_id_1: {
            let mut arr = [0;3];
            arr.copy_from_slice(&other_chunk_id_1);
            arr
        },
        done_flag,
        text: String::new(),
        magic_dollar: None,
    };

    if done_flag == 0 {
        let (rest, len_chunk) = preceded(bytes::tag(&[0x6c]), num::le_u32)(rest)?;
        let (rest, len_string) = num::u8(rest)?;
        let (rest, text) = preceded(
                                bytes::tag(&[0x01]), 
                                count(num::u8, len_string as usize),
                            )(rest)?;
        let (rest, magic_dollar) = 
            if len_chunk - len_string as u32 - 2 == 5 {
                preceded(bytes::tag(&[0x24]), pmap(num::le_u32, |p| Some(p)))(rest)?
            }
            else {
                (rest, None)
            };
        chunk.text = match String::from_utf8(text) {
        Ok(s) => s,
        Err(_) => panic!("Got bytes, but was unable to convert to valid UTF-8."),
    };
        chunk.magic_dollar = magic_dollar;
        Ok((final_input, chunk))
    }
    else {
        Ok((final_input, chunk))
    }
}

fn parse_text_backmatter(input: &[u8]) -> IResult<&[u8], TextBackmatter> {
    let (input, backmatter) =
        preceded(
            bytes::tag(&[0x1c]),
            length_value(
                num::le_u32,
                preceded(
                    bytes::take(1 as usize),
                    nom::sequence::tuple(( bytes::take(2 as usize),
                            preceded(bytes::tag(&[0x1f]), bytes::take(2 as usize)),
                            preceded(
                                bytes::tag(&[0x2c]), 
                                length_data(num::le_u32),
                            ),
                    )),
                ),
            ),
        )(input)?;
    Ok((input, TextBackmatter {
        field_0: backmatter.0.try_into().unwrap(),
        field_1: backmatter.1.try_into().unwrap(),
        field_2: backmatter.2.try_into().unwrap(),

    }))
}

fn parse_text_def(input: &[u8]) -> IResult<&[u8], TextDef> {
    let mut parsers = nom::sequence::tuple((
        preceded(bytes::tag(&[0x1f]), bytes::take(2 as usize)),
        preceded(bytes::tag(&[0x2c]), num::le_u32),
        preceded(bytes::tag(&[0x1c]), num::le_u32),
        preceded(bytes::tag(&[0x1c]), num::le_u32),
        length_count(num::u8, parse_text_chunk),
        preceded(
            bytes::tag(&[0x2c]),
            length_value(num::le_u32, many0(parse_text_backmatter)),
        ),
        preceded(
            bytes::tag(&[0x3c]),
            length_value(num::le_u32, many0(num::u8)),
        ),
        many0(num::u8),
    ));

    let (input,output) = parsers(input)?;
    let (field_0,_,_,_,texts,backmatter,unknown_sized,unknown_unsized) = output;

    Ok((input, TextDef {
        field_0: field_0.try_into().unwrap(),
        texts,
        backmatter,
        unknown_sized: unknown_sized.to_hex(unknown_sized.len()),
        unknown_unsized: unknown_unsized.to_hex(unknown_unsized.len()),
    }))
}

#[derive(Debug)]
enum Block<'a>{
    Line(Line),
    LayerDef(LayerDef),
    LayerName(LayerName),
    LayerInfo(LayerInfo),
    Unknown(&'a [u8], &'a [u8]),
}

///impl <'a> std::fmt::Display for Block<'a> {
///    // This trait requires `fmt` with this exact signature.
///    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
///        match self {
///            Block::Line(line) => {
///                write!(f, "Block::Line: {} points", line.points.len())
///            },
///            Block::LayerDef(layer) => {
///                write!(f, "Block::LayerDef: id {:?}", layer.layer_id)
///            },
///            Block::LayerName(name) => {
///                write!(f, "Block::LayerName: id {:?}, name '{}'", name.layer_id, name.name)
///            },
///            Block::LayerInfo(info) => {
///                write!(f, "Block::LayerInfo: id {:?}", info.layer_id)
///            },
///            Block::Unknown(flag, raw) => {
///                write!(f, "Block::Unknown: flag {}, bytelen {}", flag.to_hex(4), raw.len())
///            },
///        }
///    }
///}

fn parse_block(input: &[u8]) -> IResult<&[u8], Block> {
    let (input, len) = num::le_u32(input)?;
    let (input, flag) = bytes::take(4usize)(input)?;

    let (input, body) = bytes::take(len)(input)?;

    match flag {
        &[0,1,1,1] => {
            parse_layer_def(&body).map(|(_,l)| (input,Block::LayerDef(l)))
        },
        &[0,1,2,2] => {
            parse_layer_name(&body).map(|(_,l)| (input,Block::LayerName(l)))
        },
        //&[0,1,1,7] => {
        //    parse_text_def(&body).map(|(_,l)| (input,Block::TextDef(l)))
        //},
        &[0,1,1,4] => {
            parse_layer_info(&body).map(|(_,l)| (input,Block::LayerInfo(l)))
        },
        &[0,2,2,5] => {
            parse_line(&body).map(|(_,l)| (input,Block::Line(l)))
        },
        _          => {
            Ok((input, Block::Unknown(&flag, &body)))
        },
    }
}

fn parse_id_field(input: &[u8]) -> IResult<&[u8], IdField> {
    let (input, field) = bytes::take(2 as usize)(input)?;
    Ok((input, field.try_into().unwrap()))
}

fn parse_layer_info(input: &[u8]) -> IResult<&[u8], LayerInfo> {
    let mut front_parsers = nom::sequence::tuple((
        preceded(bytes::tag(&[0x1f]), parse_id_field),
        preceded(bytes::tag(&[0x2f]), parse_id_field),
        preceded(bytes::tag(&[0x3f]), parse_id_field),
        preceded(bytes::tag(&[0x4f]), parse_id_field),
        preceded(bytes::tag(&[0x54]), num::le_u32),
    ));

    let (input, front) = front_parsers(input)?;

    let mut info = LayerInfo {
        id_field_0: front.0,
        id_field_1: front.1,
        id_field_2: front.2,
        id_field_3: front.3,
        layer_id: None,
    };

    if front.4 == 0 {
        let mut back_parsers = nom::sequence::tuple((
            preceded(bytes::tag(&[0x6c]), num::le_u32), //count is always 4
            preceded(bytes::tag(&[0x02, 0x2f]), parse_id_field),
        ));
       let (input, back) = back_parsers(input)?;
       info.layer_id = Some(back.1);
       Ok((input,info))
    }
    else {
       Ok((input,info))
    }
}

fn parse_layer_name(input: &[u8]) -> IResult<&[u8], LayerName> {
    let (input, layer_id) = preceded(bytes::tag(&[0x1f]), parse_id_field)(input)?;
    let (input, _len_rest_0) = preceded(bytes::tag(&[0x2c]), num::le_u32)(input)?;
    let (input, id_field_0) = preceded(bytes::tag(&[0x1f]), parse_id_field)(input)?;
    let (input, _len_rest_1) = preceded(bytes::tag(&[0x2c]), num::le_u32)(input)?;
    let (input, len_name) = num::u8(input)?;
    let (input, name) = preceded(
                            bytes::tag(&[0x01]), 
                            count(num::u8, len_name as usize),
                        )(input)?;
    let (input, unknown_rest) = length_value(
                                    preceded(bytes::tag(&[0x3c]), num::le_u32),
                                    many0(num::u8),
                                )(input)?;

    let name_str = match String::from_utf8(name) {
        Ok(s) => s,
        Err(_) => panic!("Got bytes, but was unable to convert to valid UTF-8."),
    };
    
    Ok((input, LayerName {
        layer_id,
        id_field_0,
        name: name_str,
        unknown_rest,
    }))
}

fn parse_layer_def(input: &[u8]) -> IResult<&[u8], LayerDef> {

    let mut parsers = nom::sequence::tuple((
        preceded(bytes::tag(&[0x1f]), parse_id_field),
        preceded(bytes::tag(&[0x2f]), count(num::u8, 4)),
        length_value(
            preceded(bytes::tag(&[0x4c]), num::le_u32),
            preceded(bytes::tag(&[0x1f]), many0(num::u8)),
            )),
        );
    let (input,output) = parsers(input)?;
    
    Ok((input, LayerDef {
        layer_id: output.0,
        unknown_1: output.1,
        unknown_2: output.2,
    }))
}

fn parse_version_header(input: &[u8]) -> IResult<&[u8], u8> {
    let (input, _) = bytes::tag("reMarkable .lines file, version=")(input)?;
    let (_, version) = char::u8(input)?;
    Ok((&[], version))
}

fn parse_line(input: &[u8]) -> IResult<&[u8], Line> {

    let mut parsers = nom::sequence::tuple((
        preceded(bytes::tag(&[0x1f]),             parse_id_field),
        preceded(bytes::tag(&[0x2f]),             parse_id_field),
        preceded(bytes::tag(&[0x3f]),             parse_id_field),
        preceded(bytes::tag(&[0x4f]),             parse_id_field),
        preceded(bytes::tag(&[0x54]),             num::le_u32),
        preceded(bytes::tag(&[0x6c]),             num::le_u32),
        preceded(bytes::tag(&[0x03, 0x14]),       num::le_u32),
        preceded(bytes::tag(&[0x24]),             num::le_u32),
        preceded(bytes::tag(&[0x38,0,0,0,0]),     num::le_f32),
        bytes::tag(&[0x44,0,0,0,0]),
        preceded(
            bytes::tag(&[0x5c]),
            length_count(
                pmap(num::le_u32, |l| l/14), 
                parse_point),
            ),
        bytes::tag(&[111,0,1]),
        ));
    
    let (input, output) = parsers(input)?;

    Ok((input, Line{
        layer_id: output.0,
        line_id: output.1,
        last_line_id: output.2,
        id_field_0: output.3,
        pen_type: output.6,
        color: output.7,
        brush_size: output.8,
        points: output.10,
    }))
}

fn parse_point(input: &[u8]) -> IResult<&[u8], Point> {
    let (input, x) = num::le_f32(input)?;
    let (input, y) = num::le_f32(input)?;
    let (input, speed) = num::u8(input)?;
    let (input, _) = bytes::tag(&[0x00])(input)?;
    let (input, width) = num::u8(input)?;
    let (input, _) = bytes::tag(&[0x00])(input)?;
    let (input, direction) = num::u8(input)?;
    let (input, pressure) = num::u8(input)?;

    Ok((input, Point {
        x,
        y,
        speed,
        width,
        direction,
        pressure,
    }))
}

fn path_from_line(line: &Line) -> Path {
    const HALF_WIDTH: f32 = 702.0;
    let mut points = line.points.clone().into_iter();
    let Point{x:start_x, y:start_y,..} = points.next().unwrap();

    let mut data = Data::new().move_to((start_x+HALF_WIDTH, start_y));

    for Point {x,y,..} in points {
        data = data.line_to((x+HALF_WIDTH, y));
    };

    Path::new()
    .set("fill", "none")
    .set("stroke", "black")
    .set("stroke-width", 3)
    .set("d", data)
}

fn main() {
    let bytes: &[u8] = &read(TEST_FILE_01).unwrap();
    let (_, blocks) = many1(parse_block)(&bytes[163..]).unwrap();
    //blocks.iter().for_each(|b| println!("{:#?}",b));

    let mut document = Document::new()
        .set("viewBox", (0, 0, 1404, 1872));
    
    for block in blocks {
        if let Block::Line(line) = block {
            let path = path_from_line(&line);
            document = document.add(path);
        };
    };

    svg::save("image.svg", &document).unwrap();
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_version() {
        let bytes: &[u8] = &read(TEST_FILE_01).unwrap();
        let (_, version) = parse_version_header(&bytes[0..43]).unwrap();

        assert_eq!(version, 6);
    }

    #[test]
    fn get_line() {
        let bytes: &[u8] = &read(TEST_FILE_01).unwrap();
        let (_, block) = parse_block(&bytes[2281..]).unwrap();

        // this points in this line, in this file, has the following widths.
        if let Block::Line(line) = block {
            let widths: Vec<u8> = line.points.iter().map(|p| p.width).collect();
            assert_eq!(widths, &[37,37,38,38,39,40,41,43,43,43,44,42,41]);
        }
        else {
            panic!();
        };
    }

    #[test]
    fn get_all_blocks() {
        let bytes: &[u8] = &read(TEST_FILE_01).unwrap();
        let (_, blocks) = many1(parse_block)(&bytes[163..]).unwrap();

        assert_eq!(12, blocks.len());
    }

}