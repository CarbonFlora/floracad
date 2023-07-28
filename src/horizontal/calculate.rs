use crate::datatypes::*;

#[derive(Debug, Clone, Copy)]
pub struct HorizontalStations {
    pub pc: Station, 
    pub pi: Station,
    pub pt: Station,
}

#[derive(Debug, Clone, Copy)]
pub struct HorizontalDimensions {
    pub radius: f64,
    pub curve_length: f64,
    pub tangent: f64,
    pub long_chord: f64,
    pub middle_ordinate: f64,
    pub external: f64,
    pub curve_length_100: Angle, // Da
    pub curve_angle: Angle, 
    pub design_speed: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct HorizontalCurve {
    pub dimensions: HorizontalDimensions,
    pub stations: HorizontalStations,
}