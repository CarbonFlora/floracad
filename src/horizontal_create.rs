use std::collections::HashMap;
use std::io::{BufReader, BufRead, Error};
use std::fs::File;
use crate::angle_system::Angle;
use crate::horizontal_calculation::*;

#[derive(Debug)]
pub struct HorizontalCurve {
    dimensions: HorizontalDimensions,
    stations: HorizontalStations,
}

#[derive(Debug)]
pub struct HorizontalStations {
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
    curve_length_100: Angle, //(Da)
    curve_angle: Angle, //radians (I)
    sight_distance: Option<f64>,
}

impl HorizontalCurve {
    pub fn create(pre_given: Result<HashMap<String, String>, Error>) -> Result<HorizontalCurve, Error> { //
        let mut pre_given = pre_given?;
        let given = HorizontalCurve::nudge_create(&mut pre_given);

        let da = given.get("Da").unwrap();
        let i = given.get("I").unwrap();
        let pi = given.get("PI").unwrap();

        let dimensions = HorizontalDimensions { 
            radius: calc_radius(da), 
            curve_length: calc_curve_length(da,i), 
            tangent_distance: calc_tangent_distance(da,i), 
            long_chord: calc_long_chord(da,i), 
            middle_ordinate: calc_middle_ordinate(da,i), 
            external: calc_external(da,i), 
            curve_length_100: calc_curve_length_100(da), 
            curve_angle: calc_curve_angle(i),
            sight_distance: None,
        };
        let stations = HorizontalStations { 
            pc: calc_pc(pi, dimensions.tangent_distance), 
            pi: calc_pi(pi), 
            pt: calc_pt(pi, dimensions.tangent_distance, dimensions.curve_length)
        };

        Ok(HorizontalCurve {dimensions, stations})
    }

    fn nudge_create(given: &mut HashMap<String, String>) -> &mut HashMap<String, String> {
        if !given.contains_key("Da") {
            given.insert("Da".to_string(), radius_to_da(given.get("R").expect("missing R (radius) or Da")));
        }
        if !given.contains_key("PI") { //given R, Da, I
            if given.contains_key("PC") {
                let value = calc_pi_from_pc(given.get("I").unwrap(), given.get("R").unwrap(), given.get("PC").unwrap());

                given.insert("PI".to_string(), value);
            } else if given.contains_key("PT") {
                let value = calc_pi_from_pt(given.get("I").unwrap(), given.get("R").unwrap(), given.get("PT").unwrap());

                given.insert("PI".to_string(), value);
            } else {
                panic!("input doesn't contain PC, PI, PT");   
            }
        }

        given
    }
}