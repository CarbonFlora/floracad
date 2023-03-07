use eqsolver::single_variable::FDNewton;
use dms_coordinates::DMS;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
mod calc_dimensions;
use calc_dimensions::calc_horizontal_dimensions::*;

#[derive(Debug)]
pub enum Angle {
    decimal_degrees (f64),
    dms (DMS),
    radians (f64),
}

impl Angle {
    pub fn to_dms(self) -> Self {
        match self {
            Angle::decimal_degrees(ddeg) => Angle::dms(DMS::from_decimal_degrees(ddeg, false)),
            Angle::radians(rad) => Angle::dms(DMS::from_decimal_degrees(rad*180.0/PI, false)),
            _ => return self,
        }
    }

    pub fn to_decimal_degrees(self) -> Self {
        match self {
            Angle::dms(dms_value) => Angle::decimal_degrees(DMS::to_decimal_degrees(&dms_value)),
            Angle::radians(rad) => Angle::decimal_degrees(rad*180.0/PI),
            _ => return self,
        }
    }

    pub fn to_radians(self) -> Self {
        match self {
            Angle::dms(dms_value) => Angle::radians(DMS::to_decimal_degrees(&dms_value)*PI/180.0),
            Angle::decimal_degrees(dd_value) => Angle::radians(dd_value*PI/180.0),
            _ => return self,
        }
    }
}

#[derive(Debug)]
pub struct HorizontalCurve {
    dimensions: HorizontalDimensions,
    stations: HorizontalCriticalStations,
}

#[derive(Debug)]
pub struct HorizontalCriticalStations {
    pc: f64, 
    pi: f64,
    pt: f64,

}

#[derive(Debug)]
pub struct HorizontalDimensions {
    radius: f64,
    curve_length: f64,
    tangent_distance: f64,
    long_chord: f64,
    middle_ordinate: f64,
    external: f64,
    curve_length_100: f64, //(Da)
    curve_angle: Angle, //radians (I)
}

pub fn single_var() {
    let y = 32.0;
    //let f = |x: f64| x.exp() - 1./x; // e^x = 1/x
    let f = |x: f64| x - y*2.0; // e^x = 1/x
    let solution = FDNewton::new(f).solve(0.);
    println!("Solution: {:?}", solution);
}

pub fn parse_input() -> Result<HashMap<String, String>, Error> {
    let input = File::open("input.md")?;
    let buffered = BufReader::new(input);
    let mut arguments = HashMap::new();

    for lines in buffered.lines() {
        if let Ok(line) = lines {
            let i = line.split_once('=');
            match i {
                None => continue,
                Some(args) => arguments.insert(args.0.to_owned(), args.1.to_owned()),
            };
        }
    }

    //arguments.insert("radius", Some(3.0));
    Ok(arguments)
}

pub fn get_dimension(pi: i32, curve_length_100: f64, curve_angle: Angle, result_name: &str) -> &str {
    let mut result_dimension: &str = "";
    match result_name.to_lowercase().as_str() {
        "pc" => result_dimension = todo!(),
        "pt" => result_dimension = todo!(),
        _ => panic!("input.md is asking for a non-existent dimension."),
    }

    result_dimension
}

impl HorizontalCurve {
    pub fn create(given: Result<HashMap<String, String>, Error>) -> HorizontalCurve { //
        if let Ok(given) = given {
            let dimensions = HorizontalDimensions { 
                radius: calc_radius(given.get("Da").expect("no Da")), 
                curve_length: calc_curve_length(), 
                tangent_distance: calc_tangent_distance(), 
                long_chord: calc_long_chord(), 
                middle_ordinate: calc_middle_ordinate(), 
                external: calc_external(), 
                curve_length_100: calc_curve_length_100(), 
                curve_angle: calc_curve_angle()
            };
            let stations = HorizontalCriticalStations { 
                pc: calc_pc(), 
                pi: calc_pi(), 
                pt: calc_pt() 
            };
            
            return HorizontalCurve {dimensions, stations}
        } else {
            panic!("handle this better please");
        }
    }
} //missing is determining if there's enough information to build the HA, and building the HA itself. this is ruff.


/*The general idea:
If given any of the not Da, I, PI details, convert the given information from a .md document
to either Da, I, or PI. From there you can figure out everything else. */
