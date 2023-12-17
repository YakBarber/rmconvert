

use nom::{IResult, HexDisplay};
use nom::bytes::complete as bytes;
use nom::character::complete as char;
use nom::number::complete as num;
use nom::sequence::preceded;
use nom::multi::{length_value, length_data, length_count, many0, count};
use nom::combinator::map as pmap;

use super::types::*;

pub const TEST_FILE_01: &str = "assets/213001cb-42c0-4628-8ed0-8320c15da2a8/110b4d92-e42e-4b78-a0cb-ebd40862f2f0.rm";
pub const TEST_FILE_02: &str = "assets/213001cb-42c0-4628-8ed0-8320c15da2a8/9e0bdc4b-14cd-4d25-abb9-3ffd58d5a66e.rm";

pub fn parse_block(input: &[u8]) -> IResult<&[u8], Block> {
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
        &[0,1,1,7] => {
            parse_text_def(&body).map(|(_,l)| (input,Block::TextDef(l)))
        },
        &[0,1,1,4] => {
            parse_layer_info(&body).map(|(_,l)| (input,Block::LayerInfo(l)))
        },
        &[0,2,2,5] => {
            parse_line(&body).map(|(_,l)| (input,Block::Line(l)))
        },
        _          => {
            Ok((input, Block::Unknown(flag.to_vec(), body.to_vec())))
        },
    }
}

pub fn parse_layer_def(input: &[u8]) -> IResult<&[u8], LayerDef> {

    let mut parsers = nom::sequence::tuple((
        |i| parse_id_field(&[0x1f], i, 0x2f),
        preceded(bytes::tag(&[0x2f]), bytes::take_till(|b| b==0x4c)),
        length_value(
            preceded(bytes::tag(&[0x4c]), num::le_u32),
            preceded(bytes::tag(&[0x1f]), many0(num::u8)),
            )),
        );
    let (input,output) = parsers(input)?;
    
    Ok((input, LayerDef {
        layer_id: output.0,
        unknown_1: output.1.into(),
        unknown_2: output.2,
    }))
}

pub fn parse_layer_name(input: &[u8]) -> IResult<&[u8], LayerName> {
    let (input, layer_id) = parse_id_field(&[0x1f], input, 0x2c)?;

    let (input, subblock) = preceded(bytes::tag(&[0x2c]), length_data(num::le_u32))(input)?;
    let (subblock, id_field_0) = parse_id_field(&[0x1f], subblock, 0x2c)?;
    let (_, subblock) = preceded(bytes::tag(&[0x2c]), length_data(num::le_u32))(subblock)?;
    let (subblock, len_name) = num::u8(subblock)?;
    let (_, name) = preceded(
                            bytes::tag(&[0x01]), 
                            count(num::u8, len_name as usize),
                        )(subblock)?;

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

pub fn parse_text_def(input: &[u8]) -> IResult<&[u8], TextDef> {
    let mut parsers = nom::sequence::tuple((
        |i| parse_id_field(&[0x1f], i, 0x2c),
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

    let (input, output) = parsers(input)?;
    let (id_field_0,_,_,_,texts,backmatter,unknown_sized,unknown_unsized) = output;

    Ok((input, TextDef {
        id_field_0: id_field_0.try_into().unwrap(),
        texts,
        backmatter,
        unknown_sized: unknown_sized.to_hex(unknown_sized.len()),
        unknown_unsized: unknown_unsized.to_hex(unknown_unsized.len()),
    }))
}

pub fn parse_layer_info(input: &[u8]) -> IResult<&[u8], LayerInfo> {
    let mut front_parsers = nom::sequence::tuple((
        |i| parse_id_field(&[0x1f], i, 0x2f),
        |i| parse_id_field(&[0x2f], i, 0x3f),
        |i| parse_id_field(&[0x3f], i, 0x4f),
        |i| parse_id_field(&[0x4f], i, 0x54),
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

    //get layer id, at end of block with no terminator
    if front.4 == 0 {
        let (input, count) = preceded(bytes::tag(&[0x6c]), num::le_u32)(input)?;
        let (input, layer_id) = preceded(bytes::tag(&[0x02, 0x2f]), bytes::take(count-2))(input)?;

        info.layer_id = Some(pad_id_field(layer_id));
        Ok((input,info))
    }
    else {
       Ok((input,info))
    }
}

pub fn parse_line(input: &[u8]) -> IResult<&[u8], Line> {

    let mut id_field_parsers = nom::sequence::tuple((
        |i| parse_id_field(&[0x1f], i, 0x2f),
        |i| parse_id_field(&[0x2f], i, 0x3f),
        |i| parse_id_field(&[0x3f], i, 0x4f),
        |i| parse_id_field(&[0x4f], i, 0x54),
    ));

    let (input, id_fields) = id_field_parsers(input)?;

    let (input, done_flag) = preceded(bytes::tag(&[0x54]), num::le_u32)(input)?;

    let mut line = Line {
        layer_id: id_fields.0,
        line_id: id_fields.1,
        last_line_id: id_fields.2,
        id_field_0: id_fields.3,
        pen_type: None,
        color: None,
        brush_size: None,
        points: Vec::new(),
    };

    if done_flag==0 {
        
        let mut parsers = 
            preceded( 
                bytes::tag(&[0x6c]),
                length_value(num::le_u32,
                    nom::sequence::tuple((
                        preceded(bytes::tag(&[0x03, 0x14]),   num::le_u32),
                        preceded(bytes::tag(&[0x24]),         num::le_u32),
                        preceded(bytes::tag(&[0x38,0,0,0,0]), num::le_f32),
                        preceded(
                            bytes::tag(&[0x44,0,0,0,0,0x5c]),
                            length_count(
                                pmap(num::le_u32, |l| l/14), 
                                parse_point),
                            ),
                        bytes::tag(&[111,0,1]),
                    ))
                ),
            );
    
        let (input, details) = parsers(input)?;

        line.pen_type = Some(details.0);
        line.color = Some(details.1);
        line.brush_size = Some(details.2);
        line.points = details.3;
        Ok((input,line))
    }
    else {
        Ok((input,line))
    }
}

//---------------------------------------------

pub fn parse_text_chunk(input: &[u8]) -> IResult<&[u8], TextChunk> {
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
            pad_id_field(chunk_id)
        },
        other_chunk_id_0: {
            pad_id_field(other_chunk_id_0)
        },
        other_chunk_id_1: {
            pad_id_field(other_chunk_id_1)
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
        let (_rest, magic_dollar) = 
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

pub fn parse_text_backmatter(input: &[u8]) -> IResult<&[u8], TextBackmatter> {
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
        id_field_0: pad_id_field(backmatter.0).try_into().unwrap(),
        id_field_1: pad_id_field(backmatter.1).try_into().unwrap(),
        id_field_2: pad_id_field(backmatter.2).try_into().unwrap(),
    }))
}

pub fn pad_id_field(field: &[u8]) -> IdField {
    let vfield = {
        if field.len() == 1 {
            let v = [field.to_owned(), vec!(0), vec!(0)].concat();
            v
        } else if field.len() == 2 {
            let v = [field.to_owned(), vec!(0)].concat();
            v
        } else {
            field.to_owned()
            
        }
    };
    vfield.try_into().unwrap()
}

// this_start is the flag/delimiter of THIS field, and is discarded.
// next start is the flag/delimiter of the next field, which is not consumed.
pub fn parse_id_field<'a, 'b>(this_start: &'b [u8], input: &'a [u8], next_start: u8) -> IResult<&'a [u8], IdField> {
    let (input, field) = preceded( bytes::tag(this_start), 
                                   bytes::take_while(|b| b!=next_start)
                                   )(input)?;
            
    Ok((input, pad_id_field(field)))
}

pub fn parse_id_field_old(input: &[u8]) -> IResult<&[u8], IdField> {
    let (input, field) = bytes::take(2 as usize)(input)?;
    Ok((input, field.try_into().unwrap()))
}

pub fn parse_version_header(input: &[u8]) -> IResult<&[u8], u8> {
    let (input, _) = bytes::tag("reMarkable .lines file, version=")(input)?;
    let (_, version) = cchar::u8(input)?;
    Ok((&[], version))
}

pub fn parse_point(input: &[u8]) -> IResult<&[u8], Point> {
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

//---------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    //use crate::util::*;

    use nom::multi::many1;
    use std::fs::read;
    use std::path::PathBuf;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn can_parse_all() {
        init();
        let mut assets = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        assets.push("assets/213001cb-42c0-4628-8ed0-8320c15da2a8");
        for entry in assets.read_dir().expect("can't read assets!") {
            if let Ok(file) = entry {
                if file.path().extension().unwrap() == clap::builder::OsStr::from("rm") {
                    let bytes: &[u8] = &read(file.path()).unwrap();
                    let (_, _blocks) = many1(parse_block)(&bytes[163..]).unwrap();
                };
            };
        };
    }

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
