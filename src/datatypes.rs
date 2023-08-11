use std::f64::consts::PI;
use std::fmt;

use anyhow::{anyhow, Result};

use crate::vertical::ObstacleType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Station {
    pub value: f64,
    pub elevation: f64,
}

impl fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "STA: {:.0}+{:.2}, ELEV: {:.2}",
            (self.value / 100.0).trunc(),
            (self.value - (self.value / 100.0).trunc() * 100.0).abs(),
            self.elevation
        )?;
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
        if raw_data.is_empty() {
            return Err(anyhow!("Angle is required."));
        }
        let mut decimal_degrees = 0.;

        if raw_data.chars().any(|c| matches!(c, 'd' | '\'' | '\"')) {
            let mut parts = raw_data.trim().split_terminator(['d', '\'', '\"']);

            if raw_data.contains('d') {
                decimal_degrees += parts.next().unwrap_or("0.0").parse::<f64>()?;
            }
            if raw_data.contains('\'') {
                decimal_degrees += parts.next().unwrap_or("0.0").parse::<f64>()? / 60.0;
            }
            if raw_data.contains('\"') {
                decimal_degrees += parts.next().unwrap_or("0.0").parse::<f64>()? / 3600.0;
            }
        } else if raw_data.chars().all(|c| matches!(c, '0'..='9' | '.')) {
            decimal_degrees = raw_data.trim().parse::<f64>()?;
        } else {
            return Err(Error::ParseAngle.into());
        }

        if decimal_degrees >= 180. {
            return Err(Error::OversizedAngle.into());
        }

        Ok(Angle {
            radians: decimal_degrees * PI / 180.0,
            decimal_degrees,
        })
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
    let mut binding = string
        .split_terminator('+')
        .filter_map(|x| x.trim().parse::<f64>().ok());
    let first = binding.next().ok_or(Error::NoValue)?;
    let second = binding.next().ok_or(Error::NoPlus)?;

    if second.is_sign_negative() {
        return Err(Error::DifferentSign);
    } else if binding.count() > 0 {
        return Err(Error::ExcessiveValues);
    }

    Ok(first * 100.0 + second * first.signum())
}

pub fn coerce_elevation(string: &str) -> Result<f64, Error> {
    if string.is_empty() {
        return Err(Error::NoElevValue);
    }
    string
        .trim()
        .parse::<f64>()
        .map_err(|x| Error::ParseElevation)
}

pub fn coerce_length(string: &str) -> Result<f64, Error> {
    if string.is_empty() {
        return Err(Error::NoLenValue);
    }
    string.trim().parse::<f64>().map_err(|x| Error::ParseLength)
}

pub fn coerce_speed(string: &str) -> Result<i32, Error> {
    string.trim().parse::<i32>().map_err(|x| Error::ParseSpeed)
}

pub fn coerce_grade(string: &str) -> Result<f64, Error> {
    if string.is_empty() {
        return Err(Error::NoGradeValue);
    }

    string
        .trim()
        .trim_end_matches('%')
        .parse::<f64>()
        .map(|w| {
            if string.trim().ends_with('%') {
                w / 100.0
            } else {
                w
            }
        })
        .map_err(|x| Error::ParseGrade)
}

pub fn calc_adjustment(bools: bool) -> f64 {
    if bools {
        return 1.2;
    }
    1.
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
    /// Angles over 180 degrees are currently not supported
    #[error("Angles over 180 degrees are currently not supported.")]
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
