use anyhow::Result;

use crate::horizontal::*;
use crate::tables::get_min_sight;

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
    pub sight_distance: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct HorizontalCurve {
    pub dimensions: HorizontalDimensions,
    pub stations: HorizontalStations,
}

impl HorizontalCurve {
    pub fn is_compliant(
        &self,
        design_standard: DesignStandard,
        sight_type: SightType,
        adjustment: f64,
    ) -> Result<(bool, f64), Error> {
        let min_sight = get_min_sight(self.dimensions.design_speed, design_standard, sight_type);
        match min_sight {
            None => Err(Error::DesignSpeedLUTError),
            Some(w) => Ok((
                (self.dimensions.sight_distance >= w * adjustment),
                w * adjustment,
            )),
        }
    }
}

/// Horizontal Calculate Errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Design speed isn't specified in the manual.
    #[error("Design speed isn't specified in the manual.")]
    DesignSpeedLUTError,
}
