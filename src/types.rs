
use std::vec::Vec;
use std::default::Default;

use std::primitive::{u8,u32,f32};

use serde::Serialize;
use nom::HexDisplay;
use thiserror;

type Result<T> = std::result::Result<T, RMError>;

pub const HALF_WIDTH: f32 = 702.0;

#[derive(thiserror::Error, Debug)]
pub enum RMError {
    #[error("Bad arguments: {0}")]
    ArgsError(String),

    #[error("Can't parse: {}", .0.to_hex(.1.clone()))]
    ParseError(Vec<u8>, usize),

    #[error("SVG parsing error: {0}")]
    SvgError(#[from] svg::parser::Error),

    #[error("Something bad happened")]
    OtherError,

    #[error("IO error")]
    IoError(std::io::Error),

    #[error("Error with config")]
    ConfigError(config::ConfigError),

    #[error("Functionality not yet implemented")]
    NotImplementedError,
}

#[derive(Debug, Clone, Serialize)]
pub struct Notebook {
    pub frontmatter: Frontmatter,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct IdField(pub [u8;3]);

impl IdField {
    /// This is almost defninitely not accurate to what the tablet does,
    /// but seems to work well enough.
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

#[derive(Debug, Clone, Serialize)]
pub struct Frontmatter {
    pub version: u8,
    pub unknown: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct SimpleLine {
   pub layer_id: IdField, 
   pub line_id: IdField, 
   pub last_line_id: IdField, 
   pub id_field_0: IdField, 
   pub points: Vec<SimplePoint>,
}

#[allow(dead_code, unused_variables)]
impl SimpleLine {
    pub fn transform(&mut self, commands: &str) -> Result<()> {
        Err(RMError::NotImplementedError)
    }
}

#[derive(Debug, Clone, Serialize)]
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

#[allow(dead_code, unused_variables)]
impl Line {
    pub fn transform(&mut self, commands: &str) -> Result<()> {
        Err(RMError::NotImplementedError)
        //let tfm = tfm.strip_prefix("matrix(").unwrap();
        //let tfm = tfm.trim_end_matches(")");
        //let mut coeffs = Vec::new();
        //for coeff in tfm.split(",") {
        //    coeffs.push(coeff.trim_start().parse::<f32>().unwrap());
        //};

        //pts = pts.iter().map(|(_x, _y)| {
        //    let x = coeffs.get(0).unwrap() * _x  + coeffs.get(2).unwrap() * _y + coeffs.get(4).unwrap();
        //    let y = coeffs.get(1).unwrap() * _x  + coeffs.get(3).unwrap() * _y + coeffs.get(5).unwrap();
        //    (x,y)
        //}).collect();
    }
}

impl From<SimpleLine> for Line {
    fn from(line: SimpleLine) -> Self {
        let mut out = Line::default();
        out.layer_id = line.layer_id;
        out.line_id = line.line_id;
        out.last_line_id = line.last_line_id;
        out.id_field_0 = line.id_field_0;
        out.points = line.points.into_iter().map(Point::from).collect();
        out
    }

}

impl Default for Line {
    fn default() -> Line {
        Line {
            layer_id: IdField([0x00, 0x0b, 0x00]), 
            line_id: IdField::default(),
            last_line_id: IdField::default(),
            id_field_0: IdField::default(),
            pen_type: Some(17),
            color: Some(0),
            brush_size: Some(2.0),
            points: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct SimplePoint {
    pub x: f32,
    pub y: f32,
}

impl SimplePoint {
    pub fn to_point(&self, speed: u8, width: u8, direction: u8, pressure: u8) -> Point {
        Point {
            x: self.x,
            y: self.y,
            speed,
            width,
            direction,
            pressure,
        }
    }
}

impl From<(f32,f32)> for SimplePoint {
    fn from((x,y): (f32,f32)) -> Self {
        SimplePoint {
            x: x.clone()-HALF_WIDTH,
            y: y.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub speed: u8,
    pub width: u8,
    pub direction: u8,
    pub pressure: u8,
}

impl From<SimplePoint> for Point {
    fn from(point: SimplePoint) -> Self {
        let mut out = Point::default();
        out.x = point.x;
        out.y = point.y;
        out
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LayerDef {
    pub layer_id: IdField,
    pub unknown_1: Vec<u8>, //4 bytes
    pub unknown_2: Vec<u8>, //?? bytes
}

#[derive(Debug, Clone, Serialize)]
pub struct LayerName {
    pub layer_id: IdField,
    pub id_field_0: IdField,
    pub name: String,
    pub unknown_rest: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LayerInfo {
    pub id_field_0: IdField,
    pub id_field_1: IdField,
    pub id_field_2: IdField,
    pub id_field_3: IdField,
    pub layer_id: Option<IdField>
}

#[derive(Debug, Clone, Serialize)]
pub struct TextDef {
    pub id_field_0: IdField,
    pub texts: Vec<TextChunk>,
    pub backmatter: Vec<TextBackmatter>,
    pub unknown_sized: String,
    pub unknown_unsized: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TextChunk {
    pub chunk_id: IdField,
    pub other_chunk_id_0: IdField,
    pub other_chunk_id_1: IdField,
    pub done_flag: u32,
    pub text: String,
    pub magic_dollar: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TextBackmatter {
    pub id_field_0: IdField,
    pub id_field_1: IdField,
    pub id_field_2: IdField,
}

#[derive(Debug, Clone, Serialize)]
pub enum Block{
    Line(Line),
    LayerDef(LayerDef),
    TextDef(TextDef),
    LayerName(LayerName),
    LayerInfo(LayerInfo),
    Unknown(Vec<u8>, Vec<u8>),
}

impl TextChunk {
    pub fn to_markdown(&self) -> String {
        match self.magic_dollar {
            None => {
                todo!()
            },
            Some(4) => {
                todo!()
            },
            _ => {
                todo!()
            },
        }

    }

    pub fn to_svg(&self) -> String {
        todo!()
    }

    pub fn to_raw_text(&self) -> String {
        self.text.clone()
    }
}

impl <'a> std::fmt::Display for Block {
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

impl From<Block> for RawBytes {
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


