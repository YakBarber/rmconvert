
use std::vec::Vec;

use std::primitive::{u8,u32,f32};

use nom::HexDisplay;

#[derive(Debug, Clone)]
pub struct IdField(pub [u8;3]);

impl IdField {
    pub fn inc(&mut self, pos: usize) {
        if self.0[pos] >= 128{
            if pos == 0 {
                self.inc(1);
                self.0[pos] = 0;
            } 
            else if pos == 1 {
                self.inc(2);
                self.0[pos] = 0;
            }
            else {
                self.inc(0);
                self.0[pos] = 0;
            };
        };
        self.0[pos] = self.0[pos]+1;
    }
}

pub type RawBytes = Vec<u8>;

#[derive(Debug)]
pub struct Notebook {
    pub frontmatter: Frontmatter,
    pub layer_def: LayerDef,
    pub layer_name: LayerName,
    pub text_def: TextDef,
    pub layer_info: LayerInfo,
    pub lines: Vec<Line>,
}

#[derive(Debug)]
pub struct Frontmatter {
    pub version: u8,
    pub unknown: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Line {
   pub layer_id: IdField, 
   pub line_id: IdField, 
   pub last_line_id: IdField, 
   pub id_field_0: IdField, 
   pub pen_type: Option<u32>, 
   pub color: Option<u32>, 
   pub brush_size: Option<f32>, 
   pub points: Vec<Point>,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub speed: u8,
    pub width: u8,
    pub direction: u8,
    pub pressure: u8,
}

#[derive(Debug)]
pub struct LayerDef {
    pub layer_id: IdField,
    pub unknown_1: Vec<u8>, //4 bytes
    pub unknown_2: Vec<u8>, //?? bytes
}

#[derive(Debug)]
pub struct LayerName {
    pub layer_id: IdField,
    pub id_field_0: IdField,
    pub name: String,
    pub unknown_rest: Vec<u8>,
}

#[derive(Debug)]
pub struct LayerInfo {
    pub id_field_0: IdField,
    pub id_field_1: IdField,
    pub id_field_2: IdField,
    pub id_field_3: IdField,
    pub layer_id: Option<IdField>
}

#[derive(Debug)]
pub struct TextDef {
    pub id_field_0: IdField,
    pub texts: Vec<TextChunk>,
    pub backmatter: Vec<TextBackmatter>,
    pub unknown_sized: String,
    pub unknown_unsized: String,
}

#[derive(Debug)]
pub struct TextChunk {
    pub chunk_id: [u8;3],  //sometimes it's 3
    pub other_chunk_id_0: [u8;3], //sometimes it's 3
    pub other_chunk_id_1: [u8;3], //sometimes it's 3
    pub done_flag: u32,
    pub text: String,
    pub magic_dollar: Option<u32>,
}

#[derive(Debug)]
pub struct TextBackmatter {
    pub id_field_0: IdField,
    pub id_field_1: IdField,
    pub id_field_2: IdField,
}

#[derive(Debug)]
pub enum Block<'a>{
    Line(Line),
    LayerDef(LayerDef),
    TextDef(TextDef),
    LayerName(LayerName),
    LayerInfo(LayerInfo),
    Unknown(&'a [u8], &'a [u8]),
}

impl <'a> std::fmt::Display for Block<'a> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Block::Line(line) => {
                write!(f, "Block::Line: {} points", line.points.len())
            },
            Block::LayerDef(layer) => {
                write!(f, "Block::LayerDef: id {:?}", layer.layer_id)
            },
            Block::TextDef(text) => {
                write!(f, "Block::TextDef: id {:?}", text.id_field_0)
            },
            Block::LayerName(name) => {
                write!(f, "Block::LayerName: id {:?}, name '{}'", name.layer_id, name.name)
            },
            Block::LayerInfo(info) => {
                write!(f, "Block::LayerInfo: id {:?}", info.layer_id)
            },
            Block::Unknown(flag, raw) => {
                write!(f, "Block::Unknown: flag {}, bytelen {}", flag.to_hex(4), raw.len())
            },
        }
    }
}

impl From<Block<'_>> for RawBytes {
    fn from(value: Block) -> Self {
        match value {
            Block::Line(l) => {
                let mut raw_line = RawBytes::from(l);
                let len = raw_line.len() as u32;
                let mut out = Vec::new();
                out.append(&mut len.to_le_bytes().to_vec());
                out.append(&mut vec!(0,2,2,5));
                out.append(&mut raw_line);
                out
            },
            _ => {
                Vec::new()
            }
        }
    }
}

impl From<&[u8]> for IdField {
    fn from(value: &[u8]) -> Self {
        IdField(value.try_into().unwrap())
    }
}

impl From<Vec<u8>> for IdField {
    fn from(value: Vec<u8>) -> Self {
        IdField(value.try_into().unwrap())
    }
}

impl From<IdField> for RawBytes {
    fn from(value: IdField) -> Self {
        let mut out = Vec::new();
        out.push(value.0[0]);
        out.push(value.0[1]);
        if value.0[2] > 0 {
            out.push(value.0[2]);
        };
        out
    }
}

impl From<Point> for RawBytes {
    fn from(value: Point) -> Self {
        let list = vec!(
            value.x.to_le_bytes().to_vec(),
            value.y.to_le_bytes().to_vec(),
            vec!(value.speed),
            vec!(0),
            vec!(value.width),
            vec!(0),
            vec!(value.direction),
            vec!(value.pressure),
        );

        list.concat()
    }
}

#[allow(unused_variables)]
impl From<Line> for RawBytes {
    fn from(value: Line) -> Self {
        let mut out = Vec::new();
        out.push(0x1f);
        out.append(&mut RawBytes::from(value.layer_id));
        out.push(0x2f);
        out.append(&mut RawBytes::from(value.line_id));
        out.push(0x3f);
        out.append(&mut RawBytes::from(value.last_line_id));
        out.push(0x4f);
        out.append(&mut RawBytes::from(value.id_field_0));
        out.push(0x54);

        match value.pen_type {
            None => {},
            Some(pt) => {
                out.append(&mut vec!(0,0,0,0));
                out.push(0x6c);
                
                let mut rest = Vec::new();
                rest.append(&mut vec!(0x03, 0x14));
                rest.append(&mut pt.to_le_bytes().to_vec());
                rest.push(0x24);
                rest.append(&mut value.color.unwrap().to_le_bytes().to_vec());
                rest.append(&mut vec!(0x38,0,0,0,0));
                rest.append(&mut value.brush_size.unwrap().to_le_bytes().to_vec());
                rest.append(&mut vec!(0x44,0,0,0,0,0x5c));
                rest.append(&mut (14*value.points.len() as u32).to_le_bytes().to_vec());
                for p in value.points {
                    rest.append(&mut RawBytes::from(p));
                };

                dbg!(rest.len());
                out.append(&mut (rest.len() as u32).to_le_bytes().to_vec());
                out.append(&mut rest);
            },
        };

        out.append(&mut vec!(0x6f, 0, 1));
        
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dump_line() {
        let point = 
            Point {
                x: -351.6873779296875,
                y: 321.1515197753906,
                speed: 1,
                width: 16,
                direction: 0,
                pressure: 22,
            };

        let line = 
            Line {
               layer_id: IdField([0x00, 0x0b, 0x00]), 
               line_id: IdField([0x01, 0x0e, 0x00]), 
               last_line_id: IdField([0x00,0x00,0x00]), 
               id_field_0: IdField([0x00,0x00,0x00]), 
               pen_type: Some(17),
               color: Some(0),
               brush_size: Some(2.0),
               points: vec!(point),
            };

        let correct = vec!( 
            0x1f, 0x00, 0x0b, 0x2f, 0x01, 0x0e, 0x3f, 0x00, 
            0x00, 0x4f, 0x00, 0x00, 0x54, 0x00, 0x00, 0x00, 
            0x00, 0x6c, 0x2c, 0x00, 0x00, 0x00, 0x03, 0x14, 
            0x11, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00, 
            0x00, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
            0x00, 0x40, 0x44, 0x00, 0x00, 0x00, 0x00, 0x5c, 
            0x0e, 0x00, 0x00, 0x00, 0xfc, 0xd7, 0xaf, 0xc3, 
            0x65, 0x93, 0xa0, 0x43, 0x01, 0x00, 0x10, 0x00, 
            0x00, 0x16, 0x6f, 0x00, 0x01,
        );

        assert_eq!(correct, RawBytes::from(line));


    }
}


