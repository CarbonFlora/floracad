use std::f64::consts::PI;
use std::fmt;

use anyhow::Result;

use crate::vertical::ObstacleType;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Station {
    pub value: f64,
    pub elevation: Option<f64>,
    pub deflection: Option<f64>,
    pub chord: Option<f64>,
}

impl fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let mut cohesive_sta = format!(
            "STA: {:.0}+{:.2}",
            (self.value / 100.0).trunc(),
            (self.value - (self.value / 100.0).trunc() * 100.0).abs()
        );
        if let Some(elevation) = self.elevation {
            cohesive_sta += format!(" ELEV: {:.2}", elevation).as_str();
        }
        if let Some(deflection) = self.deflection {
            cohesive_sta += format!(" DEFL: {:.2}", deflection).as_str();
        }
        if let Some(chord) = self.chord {
            cohesive_sta += format!(" CHOR: {:.2}", chord).as_str();
        }

        write!(f, "{}", cohesive_sta.as_str())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
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

pub type ObstacleStation = (Station, ObstacleType);

#[derive(Debug, Clone, Default)]
pub struct ObstacleDetail {
    pub interval: Vec<ObstacleStation>,
}

impl fmt::Display for ObstacleDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        for station in &self.interval {
            writeln!(f, "> {:.2} {:?}", station.0, station.1)?;
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
        write!(
            f,
            "DMS: {} | DD: {:.2} | RAD: {:.2}",
            self.to_dms(),
            self.decimal_degrees,
            self.radians
        )?;
        Ok(())
    }
}

impl Angle {
    pub fn from(raw_data: &str) -> Result<Self> {
        if !raw_data.is_empty() {
            if raw_data.chars().any(|c| matches!(c, 'd' | '\'' | '\"')) {
                let parts = raw_data
                    .trim()
                    .split_terminator(['d', '\'', '\"'])
                    .collect::<Vec<&str>>();
                let mut parts_iter = parts.iter();
                let mut decimal_degrees = 0.0;
                if raw_data.contains('d') {
                    decimal_degrees += parts_iter.next().unwrap_or(&"0.0").parse::<f64>()?;
                }
                if raw_data.contains('\'') {
                    decimal_degrees += parts_iter.next().unwrap_or(&"0.0").parse::<f64>()? / 60.0;
                }
                if raw_data.contains('\"') {
                    decimal_degrees += parts_iter.next().unwrap_or(&"0.0").parse::<f64>()? / 3600.0;
                }

                if decimal_degrees >= 180. {
                    return Err(Error::OversizedAngle.into());
                }

                return Ok(Angle {
                    radians: decimal_degrees * PI / 180.0,
                    decimal_degrees,
                });
            } else if raw_data.chars().all(|c| matches!(c, '0'..='9' | '.')) {
                let decimal_degrees = raw_data.trim().parse::<f64>()?;

                if decimal_degrees >= 180. {
                    return Err(Error::OversizedAngle.into());
                }

                return Ok(Angle {
                    radians: decimal_degrees * PI / 180.0,
                    decimal_degrees,
                });
            }
        }

        Err(Error::ParseAngle.into())
    }

    pub fn to_dms(&self) -> String {
        let degrees = self.decimal_degrees.trunc();
        let minutes = ((self.decimal_degrees - degrees) * 60.0).trunc();
        let seconds = (((self.decimal_degrees - degrees) * 60.0) - minutes) * 60.0;

        format!("{:.0}d{:.0}\'{:.2}\"", degrees, minutes, seconds)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum SightType {
    #[default]
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

#[derive(Debug, Clone, Copy, Default)]
pub enum DesignStandard {
    AASHTO,
    #[default]
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

pub fn coerce_station_value(string: &str) -> Result<f64, Error> {
    let mut station_vec = vec![];
    for slice in string.split_terminator('+') {
        station_vec.push(
            slice
                .trim()
                .parse::<f64>()
                .map_err(|x| Error::ParseNonFloat)?,
        );
    }
    match station_vec.len() {
        0 => return Err(Error::NoValue),
        1 => return Err(Error::NoPlus),
        3.. => return Err(Error::ExcessiveValues),
        _ => (),
    };
    if station_vec[1].is_sign_negative() {
        return Err(Error::DifferentSign);
    }

    Ok(station_vec[0] * 100.0 + station_vec[1] * station_vec[0].signum())
}

pub fn coerce_elevation(string: &str) -> Result<f64, Error> {
    if string.is_empty() {
        Err(Error::NoElevValue)
    } else {
        let slice = string
            .trim()
            .parse::<f64>()
            .map_err(|x| Error::ParseElevation)?;

        Ok(slice)
    }
}

pub fn coerce_length(string: &str) -> Result<f64, Error> {
    if string.is_empty() {
        Err(Error::NoLenValue)
    } else {
        let slice = string
            .trim()
            .parse::<f64>()
            .map_err(|x| Error::ParseLength)?;

        Ok(slice)
    }
}

pub fn coerce_speed(string: &str) -> Result<i32, Error> {
    let slice = string
        .trim()
        .parse::<i32>()
        .map_err(|x| Error::ParseSpeed)?;

    Ok(slice)
}

pub fn coerce_grade(string: &str) -> Result<f64, Error> {
    if string.is_empty() {
        Err(Error::NoGradeValue)
    } else {
        let mut grade = string
            .trim()
            .trim_end_matches('%')
            .parse::<f64>()
            .map_err(|x| Error::ParseGrade)?;

        if string.trim().ends_with('%') {
            grade /= 100.0;
        }

        Ok(grade)
    }
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
    /// Station is misconfigured with unexpected symbol.
    #[error("Station is misconfigured with unexpected symbol.")]
    ParseNonFloat,
    /// Station is required.
    #[error("Station is required.")]
    NoValue,
    /// Station requires a "+" sign.
    #[error("Station requires a \"+\" sign.")]
    NoPlus,
    /// Station can only have one "+" sign.
    #[error("Station can only have one \"+\" sign.")]
    ExcessiveValues,
    /// Only one negative sign is required.
    #[error("Only one negative sign is required.")]
    DifferentSign,
    /// Elevation is misconfigured.
    #[error("Elevation is misconfigured with unexpected symbol.")]
    ParseElevation,
    /// Elevation is required.
    #[error("Elevation is required.")]
    NoElevValue,
    /// Length is misconfigured with unexpected symbol.
    #[error("Length is misconfigured with unexpected symbol.")]
    ParseLength,
    /// Length is required.
    #[error("Length is required.")]
    NoLenValue,
    /// Speed is misconfigured with unexpected symbol.
    #[error("Speed is misconfigured with unexpected symbol.")]
    ParseSpeed,
    /// Grade is required.
    #[error("Grade is required.")]
    NoGradeValue,
    /// Grade is misconfigured with unexpected symbol.
    #[error("Grade is misconfigured with unexpected symbol.")]
    ParseGrade,
    /// Angle is misconfigured with unexpected symbol.
    #[error("Angle is misconfigured with unexpected symbol.")]
    ParseAngle,
    /// Angle is too large.
    #[error("Angle is too large.")]
    OversizedAngle,
}

#[cfg(test)]
mod data_tests {
    use crate::datatypes::Angle;
    use anyhow::Result;

    #[test]
    fn from_angle() {
        let angles = vec![
            "10d32\'60.1\"",
            "1d0\'0\"",
            "10d",
            "10\'",
            "10\"",
            "10\'12\"",
        ];

        for angle in angles {
            match Angle::from(angle) {
                Ok(w) => println!("O: {:?}", w),
                Err(e) => println!("Failed: {} for {}", angle, e),
            }
        }
    }

    #[test]
    fn dd_dms_dd_eq() -> Result<()> {
        let angles = vec![
            "10d32\'60.1\"",
            "1d0\'0\"",
            "10d",
            "10\'",
            "10\"",
            "10\'12\"",
        ];

        for angle in angles {
            let w1 = Angle::from(angle)?;
            let w2 = w1.to_dms();
            println!("{:?} ?= {:?}", angle, w2);
        }
        Ok(())
    }
}
