use std::f64::consts::PI;
use std::fmt;

use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub struct Station {
    pub value: f64,
    pub elevation: f64,
}

impl fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "STA: {:.0}+{:.2}, ELEV: {:.2}", (self.value/100.0).trunc(), self.value-(self.value/100.0).trunc()*100.0, self.elevation)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CurveDetail {
   pub interval: Vec<Station>,
}

impl fmt::Display for CurveDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for station in &self.interval {
            writeln!(f, "> {:.2}", station)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Angle {
    pub radians: f64,
    pub decimal_degrees: f64,
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DMS:{} | DD:{:.2} | RAD:{:.2}", self.to_dms(), self.decimal_degrees, self.radians)?;
        Ok(())
    }
}

impl Angle {
    pub fn from(raw_data: &str) -> Result<Self> {
        if raw_data.len() > 0 {
            if raw_data.chars().any(|c| matches!(c,  'd'|'\''|'\"')) {
                let parts = raw_data.trim().split_terminator(['d','\'','\"']).collect::<Vec<&str>>();
                let mut parts_iter = parts.iter();
                let mut decimal_degrees = 0.0;
                if raw_data.contains('d') {
                    decimal_degrees+=parts_iter.next().unwrap_or_else(|| &"0.0").parse::<f64>()?;
                }
                if raw_data.contains('\'') {
                    decimal_degrees+=parts_iter.next().unwrap_or_else(|| &"0.0").parse::<f64>()?/60.0;
                }
                if raw_data.contains('\"') {
                    decimal_degrees+=parts_iter.next().unwrap_or_else(|| &"0.0").parse::<f64>()?/3600.0;
                }
    
                return Ok(Angle {radians: decimal_degrees*PI/180.0, decimal_degrees})
            } else if raw_data.chars().all(|c| matches!(c, '0'..='9'|'.')) {
                let decimal_degrees = raw_data.trim().parse::<f64>()?;
    
                return Ok(Angle {radians: decimal_degrees*PI/180.0, decimal_degrees})
            }
        }

        Err(Error::ParseAngle.into())
    }

    pub fn to_dms(&self) -> String {
        let degrees = self.decimal_degrees.trunc();
        let minutes = ((self.decimal_degrees-degrees)*60.0).trunc();
        let seconds = (((self.decimal_degrees-degrees)*60.0)-minutes)*60.0;

        format!("{:.0}d{:.0}\'{:.2}\"", degrees, minutes, seconds)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SightType {
    Stopping,
    Passing,
    Decision,
}

impl SightType {
    pub fn next(self) -> Self {
        match self {
            Self::Stopping => Self::Passing,
            Self::Passing => Self::Decision,
            Self::Decision => Self::Stopping,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DesignStandard {
    AASHTO,
    CALTRANS,
}

impl DesignStandard {
    pub fn next(self) -> Self {
        match self {
            Self::AASHTO => Self::CALTRANS,
            Self::CALTRANS => Self::AASHTO,
        }
    }
}

pub fn coerce_station_value(string: &String) -> Result<f64> {
    let mut station_vec = vec![];
    for slice in string.split_terminator('+') {
        station_vec.push(slice.trim().parse::<f64>().map_err(|x| Error::ParseStation)?);
    }
    if let (Some(large), Some(small)) = (station_vec.first(), station_vec.get(1)) {
        Ok(large*100.0+small)
    } else {
        Err(Error::ParseStation.into())
    }
}

pub fn coerce_elevation(string: &String) -> Result<f64> {
    let slice = string.trim().parse::<f64>().map_err(|x| Error::ParseElevation)?;

    Ok(slice)
}

pub fn coerce_length(string: &String) -> Result<f64> {
    let slice = string.trim().parse::<f64>().map_err(|x| Error::ParseLength)?;

    Ok(slice)
}

pub fn coerce_speed(string: &String) -> Result<i32> {
    let slice = string.trim().parse::<i32>().map_err(|x| Error::ParseSpeed)?;

    Ok(slice)
}

pub fn coerce_grade(string: &String) -> Result<f64> {
    let mut grade = string.trim().trim_end_matches('%').parse::<f64>().map_err(|x| Error::ParseGrade)?;

    if string.trim().ends_with('%') {
        grade /= 100.0;
    }

    Ok(grade)
}

pub fn calc_adjustment(bools: bool) -> f64 {
    let mut adjustment = 1.;
    if bools {
        adjustment += 0.2;
    }
    adjustment
}

/// Parsing errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Station is misconfigured.
    #[error("Station is misconfigured.")]
    ParseStation,
    /// Elevation is misconfigured.
    #[error("Elevation is misconfigured.")]
    ParseElevation,
    /// Length is misconfigured.
    #[error("Length is misconfigured.")]
    ParseLength,
    /// Speed is misconfigured.
    #[error("Speed is misconfigured.")]
    ParseSpeed,
    /// Grade is misconfigured.
    #[error("Grade is misconfigured.")]
    ParseGrade,
    /// Angle is misconfigured.
    #[error("Angle is misconfigured.")]
    ParseAngle,
}

#[cfg(test)]
mod data_tests {
    use crate::datatypes::Angle;
    use anyhow::Result;

    #[test]
    fn from_angle() {
        let angles = vec!["10d32\'60.1\"","1d0\'0\"","10d","10\'","10\"","10\'12\""];
        
        for angle in angles {
            match Angle::from(angle) {
                Ok(w) => println!("O: {:?}", w),
                Err(e) => println!("Failed: {} for {}", angle, e),
            }
        }
    }

    #[test]
    fn dd_dms_dd_eq() -> Result<()> {
        let angles = vec!["10d32\'60.1\"","1d0\'0\"","10d","10\'","10\"","10\'12\""];
        
        for angle in angles {
            let w1 = Angle::from(angle)?;
            let w2 = w1.to_dms();
            println!("{:?} ?= {:?}", angle, w2);
        }
        Ok(())
    }
}