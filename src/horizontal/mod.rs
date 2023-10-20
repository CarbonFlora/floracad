use anyhow::Result;
use std::f64::consts::PI;

use crate::datatypes::*;
use crate::export::*;

pub mod calculate;
pub mod display;
pub mod interval;

use self::calculate::*;

#[derive(Debug, Clone, Copy, Default)]
pub enum HorizontalStationDefinition {
    #[default]
    PI,
    PC,
    PT,
}

impl HorizontalStationDefinition {
    pub fn next(self) -> Self {
        match self {
            Self::PC => Self::PI,
            Self::PI => Self::PT,
            Self::PT => Self::PC,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum HorizontalBuildDefinition {
    #[default]
    RadiusCurveAngle,
    RadiusTangent,
}

impl HorizontalBuildDefinition {
    pub fn next(self) -> Self {
        match self {
            Self::RadiusCurveAngle => Self::RadiusTangent,
            Self::RadiusTangent => Self::RadiusCurveAngle,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct HorizontalData {
    pub input_directory: String,
    pub success_flags: [ExportSuccess; 2],
    pub input_station_method: HorizontalStationDefinition,
    pub input_build_method: HorizontalBuildDefinition,
    pub input_station: String,
    pub input_length: String,
    pub input_radius: String,
    pub input_curve_angle: String,
    pub input_tangent: String,
    pub input_station_interval: String,
    pub input_sight_type: SightType,
    pub input_design_speed: String,
    pub input_m: String,
    pub input_design_standard: DesignStandard,
    pub sustained_downgrade: bool,
    pub input_pin_station: String,
    pub pin: CurveDetail,
}

impl HorizontalData {
    fn to_dimensions(&self) -> Result<HorizontalDimensions> {
        let (
            radius,
            curve_angle,
            curve_length,
            tangent,
            external,
            middle_ordinate,
            long_chord,
            curve_length_100,
            design_speed,
            sight_distance,
        );

        match self.input_build_method {
            HorizontalBuildDefinition::RadiusCurveAngle => {
                radius = coerce_length(&self.input_radius)?;
                curve_angle = Angle::from(self.input_curve_angle.as_str())?;

                curve_length = radius * curve_angle.decimal_degrees * PI / 180.0;
                tangent = radius * (curve_angle.radians / 2.0).tan();
                external = radius * (1.0 / (curve_angle.radians / 2.0).cos() - 1.0);
                middle_ordinate = radius * (1.0 - (curve_angle.radians / 2.0).cos());
                long_chord = 2.0 * radius * (curve_angle.radians / 2.0).sin();
                curve_length_100 = Angle {
                    radians: 5729.6 / radius * PI / 180.0,
                    decimal_degrees: 5729.6 / radius,
                };
                let m = coerce_length(&self.input_m).unwrap_or_default();

                design_speed = coerce_speed(&self.input_design_speed).unwrap_or_default();
                sight_distance = radius / 28.65 * ((radius - m) / radius).acos() * 180.0 / PI;
            }
            HorizontalBuildDefinition::RadiusTangent => {
                tangent = coerce_length(&self.input_tangent)?;
                radius = coerce_length(&self.input_radius)?;

                curve_angle = Angle {
                    radians: (tangent / radius).atan() * 2.,
                    decimal_degrees: ((tangent / radius).atan() * 2.) * 180. / PI,
                };
                curve_length = radius * curve_angle.decimal_degrees * PI / 180.0;
                external = radius * (1.0 / (curve_angle.radians / 2.0).cos() - 1.0);
                middle_ordinate = radius * (1.0 - (curve_angle.radians / 2.0).cos());
                long_chord = 2.0 * radius * (curve_angle.radians / 2.0).sin();
                curve_length_100 = Angle {
                    radians: 5729.6 / radius * PI / 180.0,
                    decimal_degrees: 5729.6 / radius,
                };
                let m = coerce_length(&self.input_m).unwrap_or_default();

                design_speed = coerce_speed(&self.input_design_speed).unwrap_or_default();
                sight_distance = radius / 28.65 * ((radius - m) / radius).acos() * 180.0 / PI;
            } // _ => return Err(anyhow!("This method hasn't been implimented.")),
        }

        Ok(HorizontalDimensions {
            radius,
            curve_length,
            tangent,
            long_chord,
            middle_ordinate,
            external,
            curve_length_100,
            curve_angle,
            design_speed,
            sight_distance,
        })
    }

    fn to_stations(&self, dimensions: &HorizontalDimensions) -> Result<HorizontalStations> {
        let starting_station = Station {
            value: coerce_station_value(&self.input_station)?,
            ..Default::default()
        };

        match self.input_station_method {
            HorizontalStationDefinition::PC => Ok(HorizontalStations {
                pc: starting_station,
                pi: self.pc_to_pi(starting_station, dimensions),
                pt: self.pc_to_pt(starting_station, dimensions),
            }),
            HorizontalStationDefinition::PI => Ok(HorizontalStations {
                pc: self.pi_to_pc(starting_station, dimensions),
                pi: starting_station,
                pt: self.pi_to_pt(starting_station, dimensions),
            }),
            HorizontalStationDefinition::PT => Ok(HorizontalStations {
                pc: self.pt_to_pc(starting_station, dimensions),
                pi: self.pt_to_pi(starting_station, dimensions),
                pt: starting_station,
            }),
        }
    }

    fn pc_to_pi(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        Station {
            value: sts.value + dim.tangent,
            ..Default::default()
        }
    }

    fn pc_to_pt(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        Station {
            value: sts.value + dim.curve_length,
            ..Default::default()
        }
    }

    fn pi_to_pc(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        Station {
            value: sts.value - dim.tangent,
            ..Default::default()
        }
    }

    fn pi_to_pt(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        Station {
            value: sts.value + dim.tangent,
            ..Default::default()
        }
    }

    fn pt_to_pc(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        Station {
            value: sts.value - dim.curve_length,
            ..Default::default()
        }
    }

    fn pt_to_pi(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        Station {
            value: sts.value - dim.tangent,
            ..Default::default()
        }
    }

    pub fn to_horizontal_curve(&self) -> Result<HorizontalCurve> {
        let dimensions = self.to_dimensions()?;
        let stations = self.to_stations(&dimensions)?;

        Ok(HorizontalCurve {
            dimensions,
            stations,
        })
    }
}

#[cfg(test)]
mod hori_tests {
    use super::HorizontalData;

    #[test]
    fn h1() {
        let horizontal_data = HorizontalData {
            input_station_method: super::HorizontalStationDefinition::PI,
            input_build_method: super::HorizontalBuildDefinition::RadiusCurveAngle,
            input_station: "10284+50".to_string(),
            input_length: "600".to_string(),
            input_radius: "818.5".to_string(),
            input_curve_angle: "63d15\'34\"".to_string(),
            input_design_speed: "65".to_string(),
            input_m: "1000".to_string(),
            ..Default::default()
        };
        let hori_angle = horizontal_data.to_horizontal_curve();
        match hori_angle {
            Ok(w) => println!("O: {:#?}", w),
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn h2() {
        let horizontal_data = HorizontalData {
            input_station_method: super::HorizontalStationDefinition::PC,
            input_build_method: super::HorizontalBuildDefinition::RadiusCurveAngle,
            input_station: "100+00".to_string(),
            input_length: "600".to_string(),
            input_radius: "818.5".to_string(),
            input_curve_angle: "63d15\'34\"".to_string(),
            input_design_speed: "65".to_string(),
            input_m: "1000".to_string(),
            ..Default::default()
        };
        let hori_angle = horizontal_data.to_horizontal_curve();
        match hori_angle {
            Ok(w) => println!("O: {:#?}", w),
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn h3() {
        let horizontal_data = HorizontalData {
            input_station_method: super::HorizontalStationDefinition::PC,
            input_build_method: super::HorizontalBuildDefinition::RadiusCurveAngle,
            input_station: "100+00".to_string(),
            input_length: "600".to_string(),
            input_radius: "818.5".to_string(),
            input_curve_angle: "180".to_string(),
            ..Default::default()
        };
        let hori_angle = horizontal_data.to_horizontal_curve();
        match hori_angle {
            Ok(w) => println!("O: {:#?}", w),
            Err(e) => println!("{}", e),
        }
    }
}
