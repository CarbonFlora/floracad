use std::f64::consts::PI;

use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Copy)]
pub struct Station {
    pub value: f64,
    pub elevation: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Angle {
    pub radians: f64,
}

impl Angle {
    pub fn from(raw_data: &str) -> Result<Self> {
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

            return Ok(Angle {radians: decimal_degrees*PI/180.0})
        }

        Err(anyhow!("Failed to parse the given angle."))
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
            SightType::Stopping => SightType::Passing,
            SightType::Passing => SightType::Decision,
            SightType::Decision => SightType::Stopping,
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
            DesignStandard::AASHTO => DesignStandard::CALTRANS,
            DesignStandard::CALTRANS => DesignStandard::AASHTO,
        }
    }
}

pub fn coerce_station_value(string: String) -> Result<f64> {
    let mut station_vec = vec![];
    for slice in string.split_terminator('+') {
        station_vec.push(slice.trim().parse::<f64>().map_err(|x| anyhow!("Station is misconfigured."))?);
    }
    if let (Some(large), Some(small)) = (station_vec.first(), station_vec.get(1)) {
        Ok(large*100.0+small)
    } else {
        Err(anyhow!("Station is misconfigured."))
    }
}

pub fn coerce_elevation(string: String) -> Result<f64> {
    let slice = string.trim().parse::<f64>().map_err(|x| anyhow!("Elevation is misconfigured."))?;

    Ok(slice)
}

pub fn coerce_length(string: String) -> Result<f64> {
    let slice = string.trim().parse::<f64>().map_err(|x| anyhow!("Length is misconfigured."))?;

    Ok(slice)
}

pub fn coerce_speed(string: String) -> Result<i32> {
    let slice = string.trim().parse::<i32>().map_err(|x| anyhow!("Speed is misconfigured."))?;

    Ok(slice)
}

pub fn coerce_grade(string: String) -> Result<f64> {
    let mut grade = string.trim().trim_end_matches('%').parse::<f64>().map_err(|x| anyhow!("Grade is misconfigured."))?;

    if string.trim().ends_with('%') {
        grade /= 100.0;
    }

    Ok(grade)
}

#[cfg(test)]
mod data_tests {
    use crate::datatypes::Angle;

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
}