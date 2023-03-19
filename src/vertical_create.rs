use std::collections::HashMap;
use std::io::{BufReader, BufRead, Error};
use std::fs::File;
//use crate::angle_system::Angle;
use crate::vertical_calculation::*;

#[derive(Debug)]
pub struct VerticalCurve {
    pub dimensions: VerticalDimensions,
    pub stations: VerticalStations,
}

#[derive(Debug)]
pub struct Station {
    pub value: f64,
    pub elevation: f64,
}

impl Station {
    
}

#[derive(Debug)]
pub struct VerticalStations {
    pvc: Station, 
    pvi: Station,
    pvt: Station,
}

#[derive(Debug)]
pub struct VerticalDimensions {
    pub incoming_grade: f64,
    pub outgoing_grade: f64,
    pub curve_length: f64,
    external: f64,
    long_chord: f64,
    sight_distance: Option<f64>,
}

impl VerticalCurve {
    pub fn create(pre_given: Result<HashMap<String, String>, Error>) -> Result<VerticalCurve, Error> { //http://www.sd-w.com/channel_flow/vertical_curves/
        let mut pre_given = pre_given?;
        let given = VerticalCurve::nudge_create(&mut pre_given);

        let pvi_station = given.get("PVI-st").unwrap();
        let pvi_elevation = given.get("PVI-elev").unwrap();
        let incoming_grade = given.get("inc").unwrap();
        let outgoing_grade = given.get("out").unwrap();
        let curve_length = given.get("length").unwrap();

        let dimensions = VerticalDimensions { 
            incoming_grade: calc_incoming_grade(incoming_grade),
            outgoing_grade: calc_outgoing_grade(outgoing_grade),
            curve_length: calc_curve_length_vertical(curve_length),
            external: calc_external_vertical(incoming_grade, outgoing_grade, curve_length),
            long_chord: calc_long_chord_vertical(), //todo!()
            sight_distance: None,
        };
        let stations = VerticalStations { 
            pvc: calc_pvc(pvi_station, pvi_elevation, curve_length, dimensions.incoming_grade), //pvc = pvi - curve_length/2
            pvi: calc_pvi(pvi_station, pvi_elevation),  
            pvt: calc_pvt(pvi_station, pvi_elevation, curve_length, dimensions.outgoing_grade), //pvt = pvc + curve_length
        };

        Ok(VerticalCurve {dimensions, stations})
    }

    fn nudge_create(given: &mut HashMap<String, String>) -> &mut HashMap<String, String> {
        // if !given.contains_key("Da") {
        //     given.insert("Da".to_string(), radius_to_da(given.get("R").expect("missing R (radius) or Da")));
        // }
        if !given.contains_key("PVI-st") {
            if given.contains_key("PVC-st") {
                let value = calc_station_pvi_from_pvc(given.get("PVC-st").unwrap(), given.get("length").unwrap());

                given.insert("PVI-st".to_string(), value);
            } else if given.contains_key("PVT-st") {
                let value = calc_station_pvi_from_pvt(given.get("PVT-st").unwrap(), given.get("length").unwrap());

                given.insert("PVI-st".to_string(), value);
            } else {
                panic!("input doesn't contain a noted station.");   
            }
        }

        if !given.contains_key("PVI-elev") {
            if given.contains_key("PVC-elev") {
                let value = calc_elevation_pvi_from_pvc(given.get("PVC-elev").unwrap(), given.get("length").unwrap(), given.get("inc").unwrap());

                given.insert("PVI-elev".to_string(), value);
            } else if given.contains_key("PVT-elev") {
                let value = calc_elevation_pvi_from_pvt(given.get("PVT-elev").unwrap(), given.get("length").unwrap(), given.get("out").unwrap());

                given.insert("PVI-elev".to_string(), value);
            } else {
                panic!("input doesn't contain a noted elevation.");   
            }
        }
    
       given
    }
}