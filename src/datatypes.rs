use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Copy)]
pub struct Station {
    pub value: f64,
    pub elevation: f64,
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
    let slice = string.trim().parse::<f64>().map_err(|x| anyhow!("Elevation/Length is misconfigured."))?;

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