
use std::collections::HashMap;

use svg::node::element::path::{Data, Command, Position, Parameters};

use crate::cli::*;
use crate::types::*;
use crate::svg::{read_svg_buffer, svg_path_data_to_lines};

/// Error is rmconvert::types::Error
type Result<T> = std::result::Result<T, Error>;



fn to_xy(curr_xy: (f32, f32), pos: &Position, params: &Parameters) -> (f32, f32) {
    match pos {
        Position::Absolute => {
            let x = params.get(0).unwrap().to_owned();
            let y = params.get(1).unwrap().to_owned();
            (x,y)
        },
        Position::Relative => {
            let x = curr_xy.0 + params.get(0).unwrap().to_owned();
            let y = curr_xy.1 + params.get(1).unwrap().to_owned();
            (x,y)
        },
    }
}

// TODO: make use svg code by shoving the commands into a Data
#[allow(unused_variables)]
pub fn create_path(args: DrawArgs) -> Result<Vec<Block>> {

    let data = Data::parse(&args.svg_path.unwrap()[..]).map_err(Error::SvgError)?;

    let lines = svg_path_data_to_lines(data, IdField([0x00,0x00,0x00]), IdField([0x00,0x00,0x00]))?;


    Ok(Vec::new())
}
