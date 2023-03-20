//use dms_coordinates::DMS;
use crate::{vertical_create::Station};

pub fn calc_incoming_grade(g1: &str) -> f64 {
    g1.parse::<f64>().expect("Configure incoming grade properly.")
}

pub fn calc_outgoing_grade(g2: &str) -> f64 {
    g2.parse::<f64>().expect("Configure outgoing grade properly.")
}

pub fn calc_curve_length_vertical(curve_length: &str) -> f64 {
    curve_length.parse::<f64>().expect("Configure curve length properly.")
}

pub fn calc_external_vertical(g1: &str, g2: &str, curve_length: &str) -> f64 {
    let g1 = calc_incoming_grade(g1);
    let g2 = calc_outgoing_grade(g2);
    let curve_length = calc_curve_length_vertical(curve_length);
    let a = (g2-g1)/(2.0*curve_length);
    a*(curve_length/2.0).powf(2.0)
}

pub fn calc_long_chord_vertical() -> f64 {
    1.0
}

pub fn calc_pvc(pvi_station: &str, pvi_elevation: &str, curve_length: &str, g1: f64) -> Station {
    let pvi_station: Vec<f64> = pvi_station.split('+').map(|x| x.parse::<f64>().unwrap()).collect();
    let pvi_value = pvi_station[0]*100.0+pvi_station[1];
    let pvi_elevation = pvi_elevation.parse::<f64>().expect("Configure PVI elevation properly.");
    let curve_length = calc_curve_length_vertical(curve_length);
    let elevation_value = pvi_elevation-g1*curve_length/2.0;

    Station {value: pvi_value-curve_length/2.0, elevation: elevation_value}
}

pub fn calc_pvi(pvi_station: &str, pvi_elevation: &str) -> Station {
    let pvi_station: Vec<f64> = pvi_station.split('+').map(|x| x.parse::<f64>().unwrap()).collect();
    let pvi_value = pvi_station[0]*100.0+pvi_station[1];
    let pvi_elevation = pvi_elevation.parse::<f64>().expect("Configure PVI elevation properly.");

    Station {value: pvi_value, elevation: pvi_elevation}
}

pub fn calc_pvt(pvi_station: &str, pvi_elevation: &str, curve_length: &str, g2: f64) -> Station {
    let pvi_station: Vec<f64> = pvi_station.split('+').map(|x| x.parse::<f64>().unwrap()).collect();
    let pvi_value = pvi_station[0]*100.0+pvi_station[1];
    let pvi_elevation = pvi_elevation.parse::<f64>().expect("Configure PVI elevation properly.");
    let curve_length = calc_curve_length_vertical(curve_length);
    let elevation_value = pvi_elevation+g2*curve_length/2.0;

    Station {value: pvi_value+curve_length/2.0, elevation: elevation_value}
} 

pub fn calc_station_pvi_from_pvc(pvc_station: &str, curve_length: &str) -> String {
    let curve_length = curve_length.parse::<f64>().unwrap();
    let pvc: Vec<f64> = pvc_station.split('+').map(|x| x.parse::<f64>().unwrap()).collect();
    let pvc_value = pvc[0]*100.0+pvc[1]; //todo!(panic if pi[1] is 100 or greater || pi[2] exists)
    let pvi_value_left = ((pvc_value+curve_length/2.0)/100.0).trunc();
    let pvi_value_right = ((pvc_value+curve_length/2.0)/100.0).fract(); 

    pvi_value_left.to_string()+"+"+&pvi_value_right.to_string()
}

pub fn calc_station_pvi_from_pvt(pvt_station: &str, curve_length: &str) -> String {
    let curve_length = curve_length.parse::<f64>().unwrap();
    let pvt: Vec<f64> = pvt_station.split('+').map(|x| x.parse::<f64>().unwrap()).collect();
    let pvt_value = pvt[0]*100.0+pvt[1]; //todo!(panic if pi[1] is 100 or greater || pi[2] exists)
    let pvi_value_left = ((pvt_value+curve_length/2.0)/100.0).trunc();
    let pvi_value_right = ((pvt_value+curve_length/2.0)/100.0).fract(); 

    pvi_value_left.to_string()+"+"+&pvi_value_right.to_string()
}

pub fn calc_elevation_pvi_from_pvc(pvc_elevation: &str, curve_length: &str, g1: &str) -> String {
    dbg!(&pvc_elevation);
    let pvc_elevation = pvc_elevation.parse::<f64>().expect("Configure PVC elevation properly.");
    let curve_length = calc_curve_length_vertical(curve_length);
    let g1 = g1.parse::<f64>().unwrap();
    
    (pvc_elevation+g1*curve_length/2.0).to_string()
}

pub fn calc_elevation_pvi_from_pvt(pvt_elevation: &str, curve_length: &str, g2: &str) -> String {
    let pvt_elevation = pvt_elevation.parse::<f64>().expect("Configure PVT elevation properly.");
    let curve_length = calc_curve_length_vertical(curve_length);
    let g2 = g2.parse::<f64>().unwrap();
    
    (pvt_elevation-g2*curve_length/2.0).to_string()
}

pub fn is_within_minimum_sight_distance() -> bool {
    false
}