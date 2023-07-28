use std::f64::consts::PI;

use anyhow::{Result, anyhow};

use crate::datatypes::*;

pub mod calculate;
pub mod interval;
pub mod display;

use self::calculate::*;

#[derive(Debug, Clone, Copy)]
pub enum HorizontalStationDefinition {
    PI,
    PC,
    PT,
}

#[derive(Debug, Clone, Copy)]
pub enum HorizontalBuildDefinition {
    RadiusCurveAngle,
    RadiusTangent,
}

impl HorizontalStationDefinition {
    pub fn next(self) -> Self {
        match self {
            HorizontalStationDefinition::PC => HorizontalStationDefinition::PI,
            HorizontalStationDefinition::PI => HorizontalStationDefinition::PT,
            HorizontalStationDefinition::PT => HorizontalStationDefinition::PC,
        }
    } 
}

#[derive(Debug, Clone)]
pub struct HorizontalData {
    pub input_station_method: HorizontalStationDefinition,
    pub input_build_method: HorizontalBuildDefinition,
    pub input_station: String,
    pub input_length: String,
    pub input_radius: String,
    pub input_curve_angle: String,
    pub input_station_interval: String,
    pub input_sight_type: SightType,
    pub input_design_speed: String,
    pub input_design_standard: DesignStandard,
}

impl HorizontalData {
    fn to_dimensions(&self) -> Result<HorizontalDimensions> {
        match self.input_build_method {
            HorizontalBuildDefinition::RadiusCurveAngle => {
                let radius = coerce_length(&self.input_radius)?;
                let curve_angle = Angle::from(&self.input_curve_angle.as_str())?;
                let curve_length = radius*curve_angle.decimal_degrees*PI/180.0;
                let tangent = radius*(curve_angle.radians/2.0).tan();
                let external = radius*(1.0/(curve_angle.radians/2.0).cos()-1.0);
                let middle_ordinate = radius*(1.0-(curve_angle.radians/2.0).cos());
                let long_chord = 2.0*radius*(curve_angle.radians/2.0).sin();
                let curve_length_100 = Angle { radians: 5729.6/radius, decimal_degrees: 5729.6/radius*(180.0/PI) };
                
                let design_speed = coerce_speed(&self.input_design_speed)?;

                return Ok(HorizontalDimensions {radius, curve_length, tangent, long_chord, middle_ordinate, external, curve_length_100, curve_angle, design_speed})
            },
            _ => return Err(anyhow!("This method hasn't been implimented.")),
        }
    }

    fn to_stations(&self, dimensions: &HorizontalDimensions) -> Result<HorizontalStations> {
        let starting_station = Station { value: coerce_station_value(&self.input_station)?, elevation: 0.0 }; //todo!() this elevation is a hack

        match self.input_station_method {
            HorizontalStationDefinition::PC => {
                Ok(HorizontalStations { 
                    pc: starting_station, 
                    pi: self.pc_to_pi(starting_station, dimensions), 
                    pt: self.pc_to_pt(starting_station, dimensions), 
                })
            },
            HorizontalStationDefinition::PI => {
                Ok(HorizontalStations { 
                    pc: self.pi_to_pc(starting_station, dimensions), 
                    pi: starting_station, 
                    pt: self.pi_to_pt(starting_station, dimensions), 
                })
            },
            HorizontalStationDefinition::PT => {
                Ok(HorizontalStations { 
                    pc: self.pt_to_pc(starting_station, dimensions), 
                    pi: self.pt_to_pi(starting_station, dimensions), 
                    pt: starting_station,
                })
            },
        }   
    }

    fn pc_to_pi(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        Station { value: sts.value+dim.tangent, elevation: 0.0 }
    }

    fn pc_to_pt(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        Station { value: sts.value+dim.curve_length, elevation: 0.0 }
    }

    fn pi_to_pc(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        Station { value: sts.value-dim.tangent, elevation: 0.0 }
    }

    fn pi_to_pt(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        Station { value: sts.value+dim.tangent, elevation: 0.0 }
    }

    fn pt_to_pc(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        Station { value: sts.value-dim.curve_length, elevation: 0.0 }
    }

    fn pt_to_pi(&self, sts: Station, dim: &HorizontalDimensions) -> Station {
        Station { value: sts.value-dim.tangent, elevation: 0.0 }
    }

    pub fn to_horizontal_curve(&self) -> Result<HorizontalCurve> {
        let dimensions = self.to_dimensions()?;
        let stations = self.to_stations(&dimensions)?;

        Ok(HorizontalCurve { dimensions, stations })
    }
}

#[cfg(test)]
mod hori_tests {
    use super::HorizontalData;
    use anyhow::Result;

    #[test]
    fn hori_angle() {
        let horizontal_data = HorizontalData {
            input_station_method: super::HorizontalStationDefinition::PC,
            input_build_method: super::HorizontalBuildDefinition::RadiusCurveAngle,
            input_station: "100+00".to_string(),
            input_length: "600".to_string(),
            input_radius: "818.5".to_string(),
            input_curve_angle: "63d15\'34\"".to_string(),
            input_station_interval: "".to_string(),
            input_sight_type: crate::datatypes::SightType::Stopping,
            input_design_speed: "65".to_string(),
            input_design_standard: crate::datatypes::DesignStandard::CALTRANS,
        };
        let hori_angle = horizontal_data.to_horizontal_curve();
        match hori_angle {
            Ok(w) => println!("O: {:#?}", w),
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn hori_angle_dupe() -> Result<()> {
        let horizontal_data = HorizontalData {
            input_station_method: super::HorizontalStationDefinition::PC,
            input_build_method: super::HorizontalBuildDefinition::RadiusCurveAngle,
            input_station: "100+00".to_string(),
            input_length: "600".to_string(),
            input_radius: "818.5".to_string(),
            input_curve_angle: "63d30\'".to_string(),
            input_station_interval: "".to_string(),
            input_sight_type: crate::datatypes::SightType::Stopping,
            input_design_speed: "65".to_string(),
            input_design_standard: crate::datatypes::DesignStandard::CALTRANS,
        };
        let horizontal_data_1 = HorizontalData {
            input_station_method: super::HorizontalStationDefinition::PC,
            input_build_method: super::HorizontalBuildDefinition::RadiusCurveAngle,
            input_station: "100+00".to_string(),
            input_length: "600".to_string(),
            input_radius: "818.5".to_string(),
            input_curve_angle: "63.5d".to_string(),
            input_station_interval: "".to_string(),
            input_sight_type: crate::datatypes::SightType::Stopping,
            input_design_speed: "65".to_string(),
            input_design_standard: crate::datatypes::DesignStandard::CALTRANS,
        };
        let horizontal_data_2 = HorizontalData {
            input_station_method: super::HorizontalStationDefinition::PC,
            input_build_method: super::HorizontalBuildDefinition::RadiusCurveAngle,
            input_station: "100+00".to_string(),
            input_length: "600".to_string(),
            input_radius: "818.5".to_string(),
            input_curve_angle: "63.5".to_string(),
            input_station_interval: "".to_string(),
            input_sight_type: crate::datatypes::SightType::Stopping,
            input_design_speed: "65".to_string(),
            input_design_standard: crate::datatypes::DesignStandard::CALTRANS,
        };

        let hori_angle = horizontal_data.to_horizontal_curve()?;
        let hori_angle_1 = horizontal_data_1.to_horizontal_curve()?;
        let hori_angle_2 = horizontal_data_2.to_horizontal_curve()?;

        assert!(matches!(&hori_angle, hori_angle_1));
        assert!(matches!(hori_angle, hori_angle_2));
        Ok(())
    }

}