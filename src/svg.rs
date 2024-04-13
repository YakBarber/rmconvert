
use std::io::Write;

mod s {
    pub use svg::Document;
    pub use svg::node::element::Path;
    pub use svg::node::element::path::{Command, Data, Position, Parameters};
    pub use svg::parser::Event;
}

use crate::types::*;

type Result<T> = std::result::Result<T, RMError>;

pub const HALF_WIDTH: f32 = 702.0;
pub const WIDTH: f32 = 1404.0;
pub const HEIGHT: f32 = 1872.0;

impl TryFrom<s::Data> for SimpleLine {
    type Error = RMError;

    fn try_from(data: s::Data) -> Result<SimpleLine> {
        let base_line = SimpleLine::default();
        let mut curr_position: (f32, f32) = (0.0, 0.0);
        let mut curr_id = IdField::default();
        let last_id = IdField::default();
        let mut lines: Vec<SimpleLine> = Vec::new();

        for cmd in data.iter() {
            let mut line = base_line.clone();
            line.line_id = curr_id;
            line.last_line_id = last_id.clone();
            curr_id = last_id.clone();
            let pts = match cmd {
                s::Command::Move(pos,params) => {
                    cmd_to_xy(&mut curr_position, pos, params)
                },
                s::Command::Line(pos,params) => {
                    cmd_to_xy(&mut curr_position, pos, params)
                },
                s::Command::CubicCurve(pos,params) => {
                    let pts = cmd_to_xy(&mut curr_position, pos, params);
                    cubic_to_points(
                        pts.get(0).unwrap().clone(), 
                        pts.get(1).unwrap().clone(), 
                        pts.get(2).unwrap().clone(), 
                    )
                },
                _ => {
                    Err(RMError::NotImplementedError)?
                },
            };
            let points: Vec<SimplePoint> = pts.into_iter().map(SimplePoint::from).collect();
            line.points = points;
            lines.push(line);
        };
        Ok(SimpleLine::default())
    }
}

impl <'a>TryFrom<s::Event<'a>> for Line {
    type Error = RMError;

    fn try_from(event: s::Event) -> Result<Line> {
        match event {
            s::Event::Tag(tag, _, attributes) => {
                if tag==svg::node::element::tag::Path {
                    let data = attributes.get("d").ok_or(RMError::NotImplementedError)?;
                    let data = s::Data::parse(data)?;
                    let mut simple_line = SimpleLine::try_from(data)?;
                    match attributes.get("transform") {
                        None => {},
                        Some(tfm) => {
                            simple_line.transform(tfm)?;
                        },
                    };
                    Ok(Line::from(simple_line))
                }
                else {
                    Err(RMError::NotImplementedError)
                }
            },
            _ => {
                Err(RMError::NotImplementedError)
            },
        }
    }
}

// TODO: make `num_points` not-magic
pub fn cubic_to_points(p1: (f32,f32), p2: (f32, f32), pe: (f32, f32)) -> Vec<(f32,f32)> {
    let mut points = Vec::new();

    let num_points = 20;
    for _t in 0..num_points {
        let t = (_t as f32)/(num_points as f32);

        let x = (1.0-t)*(1.0-t)*p1.0 + 2.0*t*(1.0-t)*p2.0 + t*t*pe.0;
        let y = (1.0-t)*(1.0-t)*p1.1 + 2.0*t*(1.0-t)*p2.1 + t*t*pe.1;
        points.push((x,y));
    };
    points
}

fn cmd_to_xy(curr_pos: &mut (f32, f32), pos: &s::Position, params: &s::Parameters) -> Vec<(f32, f32)> {

    let mut out = Vec::new();

    for _i in 0..params.len()/2 {
        let i = _i*2;

        let this = match pos {
            s::Position::Absolute => {
                let x = params.get(i).unwrap().to_owned();
                let y = params.get(i+1).unwrap().to_owned();
                (x,y)
            },
            s::Position::Relative => {
                let x = curr_pos.0 + params.get(i).unwrap().to_owned();
                let y = curr_pos.1 + params.get(i+1).unwrap().to_owned();
                (x,y)
            },
        };

        out.push(this);
    };
    match out.last() {
        None => {},
        Some(last) => {
            _ = std::mem::replace(curr_pos, last.to_owned());
        },
    };
    out
}


pub fn path_from_line(line: &Line) -> Option<s::Path> {
    let mut points = line.points.clone().into_iter();
    let Point{x:start_x, y:start_y,..} = points.next()?;

    let mut data = s::Data::new().move_to((start_x+HALF_WIDTH, start_y));

    for Point {x,y,..} in points {
        data = data.line_to((x+HALF_WIDTH, y));
    };

    Some(
        s::Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data)
    )
}

pub fn create_border_path() -> s::Path {
    let data = 
        s::Data::new()
        .move_to((0,0))
        .line_to((WIDTH, 0.0))
        .line_to((WIDTH, HEIGHT))
        .line_to((0.0, HEIGHT))
        .close();

    s::Path::new()
        .set("fill", "none")
        .set("stroke", "gray")
        .set("stroke-width", 1)
        .set("d", data)
}

fn prepare_svg<I>(paths: I) -> s::Document
    where I: IntoIterator<Item=s::Path>
{
    let margin = 0.0; //50.0;
    let mut document = s::Document::new()
        .set("viewBox", (-margin, -margin, WIDTH+margin, HEIGHT+margin));
    
    for p in paths {
        document = document.add(p);
    };
    document
}

pub fn write_svg<I, T>(paths: I, filepath: T) -> std::io::Result<()> 
    where I: IntoIterator<Item=s::Path>,
          T: AsRef<std::path::Path>,
{
    let document = prepare_svg(paths);

    svg::save(filepath, &document)
}

pub fn write_svg_to_stdout<I>(paths: I) -> std::io::Result<()> 
    where I: IntoIterator<Item=s::Path>
{
    let document = prepare_svg(paths);
    let out_bytes = document.to_string().into_bytes();
    std::io::stdout().write_all(&out_bytes)
}


/// Read an SVG file into a reMarkable-style struct (rmconvert::types::Line).
///
/// TODO: make this interface better
pub fn read_svg_file<P: AsRef<std::path::Path>>(filepath: P) -> Result<Vec<Line>> {
    let mut content = String::new();
    let events = match svg::open(filepath, &mut content){
        Ok(parser) => {
            parser
        },
        Err(error) => {
            return Err(RMError::IoError(error))
        },
    };
    let mut lines = Vec::new();
    for event in events {
        lines.push(Line::try_from(event)?);
    };
    Ok(lines)
}

pub fn read_svg_buffer<'a>(svg_buf: &str) -> Result<Vec<Line>> {
    let events = match svg::read(svg_buf) {
        Ok(parser) => {
            parser
        },
        Err(error) => {
            return Err(RMError::IoError(error))
        },
    };

    let mut lines = Vec::new();
    for event in events {
        lines.push(Line::try_from(event)?);
    };
    Ok(lines)
    
}

