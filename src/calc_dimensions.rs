pub mod calc_horizontal_dimensions {
    use dms_coordinates::{DMS};

    use crate::Angle;

    pub fn calc_radius(da: &String) -> f64 {
        let da = Angle::create_dms(da).to_decimal_degrees().value();
        //println!("da: {:?}", da.value());

        5729.58/da
    }

    pub fn calc_curve_length(da: &String, i: &String) -> f64 {
        let da = Angle::create_dms(da).to_decimal_degrees().value();
        let i = Angle::create_dms(i).to_decimal_degrees().value();
        
        100.0*i/da
    }

    pub fn calc_tangent_distance(da: &String, i: &String) -> f64 {
        let radius = 5729.58/Angle::create_dms(da).to_decimal_degrees().value();
        let i = Angle::create_dms(i).to_radians().value();

        radius*(i/2.0).tan()
    }

    pub fn calc_long_chord(da: &String, i: &String) -> f64 {
        let radius = 5729.58/Angle::create_dms(da).to_decimal_degrees().value();
        let i = Angle::create_dms(i).to_radians().value();

        2.0*radius*(i/2.0).sin()
    }

    pub fn calc_middle_ordinate(da: &String, i: &String) -> f64 {
        let radius = 5729.58/Angle::create_dms(da).to_decimal_degrees().value();
        let i = Angle::create_dms(i).to_radians().value();

        radius*(1.0-(i/2.0).cos())
    }

    pub fn calc_external(da: &String, i: &String) -> f64 {
        let radius = 5729.58/Angle::create_dms(da).to_decimal_degrees().value();
        let i = Angle::create_dms(i).to_radians().value();
        let tan_dist = radius*(i/2.0).tan();

        tan_dist*(i/4.0).tan()
    }

    pub fn calc_curve_length_100(da: &String) -> Angle {
        Angle::create_dms(da)
    }

    pub fn calc_curve_angle(i: &String) -> Angle {
        Angle::create_dms(i)
    }

    pub fn calc_pc(pi: &String, tan_dist: f64) -> f64 {
        let pi: Vec<f64> = pi.split('+').map(|x| x.parse::<f64>().unwrap()).collect();
        let pi_value = pi[0]*100.0+pi[1]; //todo!(panic if pi[1] is 100 or greater || pi[2] exists)
        
        pi_value-tan_dist
    }
    pub fn calc_pi(pi: &String) -> f64 {
        let pi: Vec<f64> = pi.split('+').map(|x| x.parse::<f64>().unwrap()).collect();
        
        pi[0]*100.0+pi[1]
    }
    pub fn calc_pt(pi: &String, tan_dist: f64, curve_length: f64) -> f64 {
        let pi: Vec<f64> = pi.split('+').map(|x| x.parse::<f64>().unwrap()).collect();
        let pi_value = pi[0]*100.0+pi[1]; //todo!(panic if pi[1] is 100 or greater || pi[2] exists)
        
        pi_value-tan_dist+curve_length
    }

    pub fn radius_to_da(radius: &String) -> String {
        let radius = radius.parse::<f64>().unwrap();
        let val = DMS::from_decimal_degrees(5729.58/radius, false);
        let rstring = String::from({(val.degrees).to_string()}+"d"+{&(val.minutes).to_string()}+"'"+{&(val.seconds).to_string()}+"\"");
        rstring
    }
}