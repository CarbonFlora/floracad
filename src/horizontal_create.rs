use std::collections::HashMap;
use std::io::Error;
use crate::angle_system::Angle;
use crate::horizontal_calculation::*;
use crate::sight_distance::*;

#[derive(Debug, Clone, Copy)]
pub struct HorizontalCurve {
    pub dimensions: HorizontalDimensions,
    pub stations: HorizontalStations,
}

#[derive(Debug, Clone, Copy)]
pub struct HorizontalStations {
    pub pc: f64, 
    pub pi: f64,
    pub pt: f64,

}

#[derive(Debug, Clone, Copy)]
pub struct HorizontalDimensions {
    radius: f64,
    pub curve_length: f64,
    tangent_distance: f64,
    long_chord: f64,
    middle_ordinate: f64,
    external: f64,
    curve_length_100: Angle, //(Da)
    curve_angle: Angle, //radians (I)
    pub sight_distance: Option<f64>,
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

        let mut curve = HorizontalCurve {dimensions, stations};
        curve.dimensions.sight_distance = Some(curve.calc_sight_distance());
        
        if Curve::HorizontalCurve(curve).examine_functional(SightType::Stopping, 65, false).values().all(|b| *b) { //todo!() make interact
            println!("Curve passes all relevant inspections.");
        } else {
            println!("Curve fails all relevant inspections.");
        }
        Ok(curve)
        //Ok(HorizontalCurve {dimensions, stations})
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

    fn calc_sight_distance(&self) -> f64 { //untested! todo!()
        let radius = self.dimensions.radius;
        let middle_ordinate = self.dimensions.middle_ordinate;

        //dbg!(radius/28.65*(((radius-middle_ordinate)/radius).cos()));
        radius/28.65*(((radius-middle_ordinate)/radius).acos().to_degrees())
    }
}