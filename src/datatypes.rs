use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Copy)]
pub struct Station {
    pub value: f64,
    pub elevation: f64,
}

pub fn coerce_station_value(string: String) -> Result<f64> {
    let mut station_vec = vec![];
    for slice in string.split_terminator('+') {
        station_vec.push(slice.trim().parse::<f64>()?);
    }
    if let (Some(large), Some(small)) = (station_vec.get(0), station_vec.get(1)) {
        Ok(large*100.0+small)
    } else {
        Err(anyhow!("Station is misconfigured."))
    }
}

pub fn coerce_elevation(string: String) -> Result<f64> {
    let slice = string.trim().parse::<f64>()?;

    Ok(slice)
}

pub fn coerce_grade(string: String) -> Result<f64> {
    let mut grade = string.trim().trim_end_matches('%').parse::<f64>()?;

    if string.trim().chars().last()==Some('%') {
        grade = grade/100.0;
    }

    Ok(grade)
}