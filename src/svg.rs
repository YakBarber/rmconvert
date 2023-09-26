
use std::io::Write;

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::{Command, Data, Position, Parameters};
use svg::parser::Event;

use crate::types::*;

pub const HALF_WIDTH: f32 = 702.0;
pub const WIDTH: f32 = 1404.0;
pub const HEIGHT: f32 = 1872.0;

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

pub fn path_from_line(line: &Line) -> Option<Path> {
    let mut points = line.points.clone().into_iter();
    let Point{x:start_x, y:start_y,..} = points.next()?;

    let mut data = Data::new().move_to((start_x+HALF_WIDTH, start_y));

    for Point {x,y,..} in points {
        data = data.line_to((x+HALF_WIDTH, y));
    };

    Some(
        Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data)
    )
}

pub fn create_border_path() -> Path {
    let data = 
        Data::new()
        .move_to((0,0))
        .line_to((WIDTH, 0.0))
        .line_to((WIDTH, HEIGHT))
        .line_to((0.0, HEIGHT))
        .close();

    Path::new()
        .set("fill", "none")
        .set("stroke", "gray")
        .set("stroke-width", 1)
        .set("d", data)
}

fn prepare_svg<I>(paths: I) -> Document
    where I: IntoIterator<Item=Path>
{
    let margin = 0.0; //50.0;
    let mut document = Document::new()
        .set("viewBox", (-margin, -margin, WIDTH+margin, HEIGHT+margin));
    
    for p in paths {
        document = document.add(p);
    };
    document
}

pub fn write_svg<I, T>(paths: I, filepath: T) -> std::io::Result<()> 
    where I: IntoIterator<Item=Path>,
          T: AsRef<std::path::Path>,
{
    let document = prepare_svg(paths);

    svg::save(filepath, &document)
}



/// Read an SVG file into a reMarkable-style struct (rmconvert::types::Line).
///
/// TODO: make this interface better
pub fn read_svg_file<P: AsRef<std::path::Path>>(filepath: P) -> std::io::Result<Vec<Line>> {
    let mut content = String::new();
    let events = svg::open(filepath, &mut content)?;
    parse_svg_to_lines(events, IdField([0x00,0x00,0x00]), IdField([0x00,0x00,0x00]))
}

pub fn read_svg_buffer<'a>(svg_buf: &str) -> std::io::Result<Vec<Line>> {
    let events = svg::read(svg_buf)?;
    parse_svg_to_lines(events, IdField([0x00,0x00,0x00]), IdField([0x00,0x00,0x00]))
    
}

fn parse_svg_to_lines<'a, I, E>(svg_events: I, start_id: IdField, prev_id: IdField) -> Result<Vec<Line>, E> 
    where I: IntoIterator<Item=Event<'a>>,
{
    let base_line = Line {
        layer_id: IdField([0x00, 0x0b, 0x00]), 
        line_id: IdField([0x00, 0x00, 0x00]),
        last_line_id: IdField([0x00, 0x00, 0x00]),
        id_field_0: IdField([0x00, 0x00, 0x00]), 
        pen_type: Some(17),
        color: Some(0),
        brush_size: Some(2.0),
        points: Vec::new(),
    };

    let mut curr_position: (f32, f32) = (0.0, 0.0);
    let mut curr_id = start_id;
    let mut last_id = prev_id;
    let mut lines: Vec<Line> = Vec::new(); //my Line, not svg::_::Line

    for event in svg_events {
        match event {
            Event::Tag(tag, _, attributes) => {
                if tag!="path" {
                    continue;
                };

                let mut line = base_line.clone();
                line.last_line_id = last_id.clone();
                line.line_id = curr_id.clone();

                curr_id.inc(1);
                last_id.inc(1);

                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();

                for cmd in data.iter() {

                    let mut pts = Vec::new();

                    match cmd {
                        Command::Move(pos,params) => {
                            pts = cmd_to_xy(curr_position, pos, params);
                        },
                        Command::Line(pos,params) => {
                            pts = cmd_to_xy(curr_position, pos, params);
                        },
                        Command::CubicCurve(pos,params) => {
                            pts = cmd_to_xy(curr_position, pos, params);
                            pts = cubic_to_points(
                                pts.get(0).unwrap().clone(), 
                                pts.get(1).unwrap().clone(), 
                                pts.get(2).unwrap().clone(), 
                                );

                        },
                        _ => {},
                    };

                    match attributes.get("transform") {
                        None => {},
                        Some(tfm) => {
                            let tfm = tfm.strip_prefix("matrix(").unwrap();
                            let tfm = tfm.trim_end_matches(")");
                            let mut coeffs = Vec::new();
                            for coeff in tfm.split(",") {
                                coeffs.push(coeff.trim_start().parse::<f32>().unwrap());
                            };

                            pts = pts.iter().map(|(_x, _y)| {
                                let x = coeffs.get(0).unwrap() * _x  + coeffs.get(2).unwrap() * _y + coeffs.get(4).unwrap();
                                let y = coeffs.get(1).unwrap() * _x  + coeffs.get(3).unwrap() * _y + coeffs.get(5).unwrap();
                                (x,y)
                            }).collect();
                        },
                    };

                    curr_position = pts.last().unwrap().clone();

                    for (x,y) in pts.iter() {
                        let point = Point {
                            x: x.clone()-HALF_WIDTH,
                            y: y.clone(),
                            speed: 1,
                            width: 16,
                            direction: 0,
                            pressure: 22,
                        };

                        line.points.push(point);
                    };
                };
                lines.push(line);
            },
            _ => {},
            };
        };
    Ok(lines)
}

fn cmd_to_xy(curr_pos: (f32, f32), pos: &Position, params: &Parameters) -> Vec<(f32, f32)> {

    let mut out = Vec::new();

    for _i in 0..params.len()/2 {
        let i = _i*2;

        let this = match pos {
            Position::Absolute => {
                let x = params.get(i).unwrap().to_owned();
                let y = params.get(i+1).unwrap().to_owned();
                (x,y)
            },
            Position::Relative => {
                let x = curr_pos.0 + params.get(i).unwrap().to_owned();
                let y = curr_pos.1 + params.get(i+1).unwrap().to_owned();
                (x,y)
            },
        };

        out.push(this);
    };
    out
}
