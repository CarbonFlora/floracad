use core::f64::consts::PI;
use dms_coordinates::DMS;
use dms_coordinates::Bearing::*;

#[derive(Debug, Clone, Copy)]
pub enum Angle {
    DecimalDegrees (f64),
    Dms (DMS),
    Radians (f64),
}

impl Angle {
    pub fn create_dms(line: &str) -> Angle {
        let delimiters = "'\"d";
        let parts: Vec<&str> = line.split(|c| delimiters.contains(c)).collect();
        
        Angle::Dms(DMS { 
            degrees: parts[0].parse::<i32>().unwrap(), 
            minutes: parts[1].parse::<i32>().unwrap(), 
            seconds: parts[2].parse::<f64>().unwrap(), 
            bearing: East 
        })
    }

    pub fn value(&self) -> f64 {
        match self {
            Angle::DecimalDegrees(n) => *n,
            Angle::Radians(n) => *n,
            Angle::Dms(_n) => panic!("Convert dms to radians or decimal degrees first."),
        }
    }

    pub fn to_dms(self) -> Self {
        match self {
            Angle::DecimalDegrees(ddeg) => Angle::Dms(DMS::from_decimal_degrees(ddeg, false)),
            Angle::Radians(rad) => Angle::Dms(DMS::from_decimal_degrees(rad*180.0/PI, false)),
            _ => self,
        }
    }

    pub fn to_decimal_degrees(self) -> Self {
        match self {
            Angle::Dms(dms_value) => Angle::DecimalDegrees(DMS::to_decimal_degrees(&dms_value)),
            Angle::Radians(rad) => Angle::DecimalDegrees(rad*180.0/PI),
            _ => self,
        }
    }

    pub fn to_radians(self) -> Self {
        match self {
            Angle::Dms(dms_value) => Angle::Radians(DMS::to_decimal_degrees(&dms_value)*PI/180.0),
            Angle::DecimalDegrees(dd_value) => Angle::Radians(dd_value*PI/180.0),
            _ => self,
        }
    }
}