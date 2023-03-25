use std::collections::HashMap;
use std::io::{BufReader, BufRead, Error, ErrorKind};
use std::fs::File;
use crate::sight_distance::{calc_min_sight_distance, SightType, parse_table};
//use crate::angle_system::Angle;
use crate::vertical_calculation::*;
use crate::sight_distance::*;

#[derive(Debug, Clone, Copy)]
pub struct VerticalCurve {
    pub dimensions: VerticalDimensions,
    pub stations: VerticalStations,
}

#[derive(Debug, Clone, Copy)]
pub struct Station {
    pub value: f64,
    pub elevation: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct VerticalStations {
    pvc: Station, 
    pvi: Station,
    pvt: Station,
}

#[derive(Debug, Clone, Copy)]
pub struct VerticalDimensions {
    pub incoming_grade: f64,
    pub outgoing_grade: f64,
    pub curve_length: f64,
    external: f64,
    pub sight_distance: Option<f64>,
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
            sight_distance: None,
        };
        let stations = VerticalStations { 
            pvc: calc_pvc(pvi_station, pvi_elevation, curve_length, dimensions.incoming_grade), //pvc = pvi - curve_length/2
            pvi: calc_pvi(pvi_station, pvi_elevation),  
            pvt: calc_pvt(pvi_station, pvi_elevation, curve_length, dimensions.outgoing_grade), //pvt = pvc + curve_length
        };

        let mut curve = VerticalCurve {dimensions, stations};
        curve.dimensions.sight_distance = Some(curve.calc_sight_distance());
        
        if Curve::VerticalCurve(curve).examine_functional(SightType::Stopping, 65, false).values().all(|b| *b) { //todo!() make interact
            println!("Curve passes all relevant inspections.");
        } else {
            println!("Curve fails all relevant inspections.");
        }
        Ok(curve)
        //Err(Error::new(ErrorKind::Other, "sight distance functions configured incorrectly."))
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

    //from HDM, assuming 3.5 ft driver eye height, 0.5ft obstruction height.
    fn calc_sight_distance(&self) -> f64 { //untested! todo!()
        let grade_diff = (self.dimensions.outgoing_grade-self.dimensions.incoming_grade).abs();
        let curve_length = self.dimensions.curve_length;
        
        if self.dimensions.incoming_grade > self.dimensions.outgoing_grade { //crest curve handling
            let eq_sight_1 = ((grade_diff*curve_length+1329.0)/(2.0*grade_diff)).abs(); //fails on no grade difference.
            let eq_sight_2 = (1329.0f64.sqrt()*curve_length.sqrt()/grade_diff.sqrt()).abs(); //fails on no grade difference.
            //dbg!(&eq_sight_1);
            //dbg!(&eq_sight_2);
            if eq_sight_1 > curve_length {
                return eq_sight_1;
            } else if eq_sight_2 < curve_length {
                return eq_sight_2;
            } else {
                panic!("vertical curve crest curve handling failed.");
            }
        } else if self.dimensions.incoming_grade < self.dimensions.outgoing_grade {
            let eq_sight_1 = ((2.0*(grade_diff*curve_length+400.0))/(4.0*grade_diff-7.0)).abs();
            let eq_sight_2 = ((1600.0*curve_length.sqrt())/((6400.0*grade_diff+49.0*curve_length).sqrt()+7.0*curve_length.sqrt())).abs();
            //dbg!(&eq_sight_1);
            //dbg!(&eq_sight_2);
            if eq_sight_1 > curve_length {
                return eq_sight_1;
            } else if eq_sight_2 < curve_length {
                return eq_sight_2;
            } else {
                panic!("vertical curve sag curve handling failed.");
            }
        } else {
            panic!("failed stopping sight distance calculations.");
        }
    }
}