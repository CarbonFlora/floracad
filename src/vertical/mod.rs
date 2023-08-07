use anyhow::Result;

use crate::{datatypes::*, export::ExportSuccess};

pub mod calculate;
pub mod display;
pub mod interval;

use self::calculate::*;

#[derive(Debug, Clone, Copy, Default)]
pub enum VerticalDefinition {
    #[default]
    PVI,
    PVC,
    PVT,
}

impl VerticalDefinition {
    pub fn next(self) -> Self {
        match self {
            VerticalDefinition::PVC => VerticalDefinition::PVI,
            VerticalDefinition::PVI => VerticalDefinition::PVT,
            VerticalDefinition::PVT => VerticalDefinition::PVC,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ObstacleType {
    #[default]
    Above, //like a bridge
    Below, //like a underground water pipe
}

impl ObstacleType {
    pub fn next(self) -> Self {
        match self {
            ObstacleType::Above => ObstacleType::Below,
            ObstacleType::Below => ObstacleType::Above,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct VerticalData {
    pub input_directory: String,
    pub success_flags: [ExportSuccess; 3],
    pub input_method: VerticalDefinition,
    pub input_station: String,
    pub input_elevation: String,
    pub input_incoming_grade: String,
    pub input_outgoing_grade: String,
    pub input_length: String,
    pub input_station_interval: String,
    pub input_sight_type: SightType,
    pub input_design_speed: String,
    pub input_design_standard: DesignStandard,
    pub sustained_downgrade: bool,
    pub input_obstacle_station: String,
    pub input_obstacle_elevation: String,
    pub input_obstacle_type: ObstacleType,
    pub obstacles: ObstacleDetail,
}

impl VerticalData {
    fn to_dimensions(&self) -> Result<VerticalDimensions> {
        let incoming_grade = coerce_grade(&self.input_incoming_grade)?;
        let outgoing_grade = coerce_grade(&self.input_outgoing_grade)?;
        let curve_length = coerce_length(&self.input_length)?;
        let a = (outgoing_grade - incoming_grade) / (2.0 * curve_length);
        let external = a * (curve_length / 2.0).powi(2);
        let design_speed = coerce_speed(&self.input_design_speed).unwrap_or_default();
        let sustained_downgrade = self.sustained_downgrade;

        Ok(VerticalDimensions {
            incoming_grade,
            outgoing_grade,
            curve_length,
            external,
            design_speed,
            sustained_downgrade,
        })
    }

    fn to_stations(&self, dimensions: &VerticalDimensions) -> Result<VerticalStations> {
        let starting_station = Station {
            value: coerce_station_value(&self.input_station)?,
            elevation: coerce_elevation(&self.input_elevation)?,
        };

        match self.input_method {
            VerticalDefinition::PVC => Ok(VerticalStations {
                pvc: starting_station,
                pvi: self.pvc_to_pvi(starting_station, dimensions),
                pvt: self.pvc_to_pvt(starting_station, dimensions),
            }),
            VerticalDefinition::PVI => Ok(VerticalStations {
                pvc: self.pvi_to_pvc(starting_station, dimensions),
                pvi: starting_station,
                pvt: self.pvi_to_pvt(starting_station, dimensions),
            }),
            VerticalDefinition::PVT => Ok(VerticalStations {
                pvc: self.pvt_to_pvc(starting_station, dimensions),
                pvi: self.pvt_to_pvi(starting_station, dimensions),
                pvt: starting_station,
            }),
        }
    }

    fn pvc_to_pvi(&self, sts: Station, dim: &VerticalDimensions) -> Station {
        Station {
            value: sts.value + dim.curve_length / 2.0,
            elevation: sts.elevation + dim.incoming_grade * dim.curve_length / 2.0,
        }
    }

    fn pvc_to_pvt(&self, sts: Station, dim: &VerticalDimensions) -> Station {
        Station {
            value: sts.value + dim.curve_length,
            elevation: sts.elevation
                + dim.incoming_grade * dim.curve_length / 2.0
                + dim.outgoing_grade * dim.curve_length / 2.0,
        }
    }

    fn pvi_to_pvc(&self, sts: Station, dim: &VerticalDimensions) -> Station {
        Station {
            value: sts.value - dim.curve_length / 2.0,
            elevation: sts.elevation - dim.incoming_grade * dim.curve_length / 2.0,
        }
    }

    fn pvi_to_pvt(&self, sts: Station, dim: &VerticalDimensions) -> Station {
        Station {
            value: sts.value + dim.curve_length / 2.0,
            elevation: sts.elevation + dim.outgoing_grade * dim.curve_length / 2.0,
        }
    }

    fn pvt_to_pvc(&self, sts: Station, dim: &VerticalDimensions) -> Station {
        Station {
            value: sts.value - dim.curve_length,
            elevation: sts.elevation
                - dim.incoming_grade * dim.curve_length / 2.0
                - dim.outgoing_grade * dim.curve_length / 2.0,
        }
    }

    fn pvt_to_pvi(&self, sts: Station, dim: &VerticalDimensions) -> Station {
        Station {
            value: sts.value - dim.curve_length / 2.0,
            elevation: sts.elevation - dim.outgoing_grade * dim.curve_length / 2.0,
        }
    }

    pub fn to_vertical_curve(&self) -> Result<VerticalCurve> {
        let dimensions = self.to_dimensions()?;
        let stations = self.to_stations(&dimensions)?;

        Ok(VerticalCurve {
            dimensions,
            stations,
        })
    }
}

#[cfg(test)]
mod vertical_tests {

    use crate::datatypes::Station;

    use super::{VerticalData, VerticalDefinition};

    #[test]
    fn v1() {
        let data = VerticalData {
            input_method: VerticalDefinition::PVI,
            input_station: "10284+50".to_string(),
            input_elevation: "1001.38".to_string(),
            input_incoming_grade: "0.44%".to_string(),
            input_outgoing_grade: "-0.57%".to_string(),
            input_length: "500".to_string(),
            ..Default::default()
        };
        let curve = data.to_vertical_curve().unwrap();
        assert_eq!(
            curve.stations.pvc,
            Station {
                value: 1028200.,
                elevation: 1000.28
            }
        );
        assert_eq!(
            curve.stations.pvt,
            Station {
                value: 1028700.,
                elevation: 999.955
            }
        );
    }

    #[test]
    fn v2() {
        let data = VerticalData {
            input_method: VerticalDefinition::PVI,
            input_station: "-10284+50".to_string(),
            input_elevation: "1001.38".to_string(),
            input_incoming_grade: "0.44%".to_string(),
            input_outgoing_grade: "-0.57%".to_string(),
            input_length: "500".to_string(),
            ..Default::default()
        };
        let curve = data.to_vertical_curve().unwrap();
        assert_eq!(
            curve.stations.pvc,
            Station {
                value: -1028700.,
                elevation: 1000.28
            }
        );
        assert_eq!(
            curve.stations.pvt,
            Station {
                value: -1028200.,
                elevation: 999.955
            }
        );
    }

    #[test]
    fn v3() {
        let data = VerticalData {
            input_method: VerticalDefinition::PVI,
            input_station: "-0+0".to_string(),
            input_elevation: "1001.38".to_string(),
            input_incoming_grade: "0.44%".to_string(),
            input_outgoing_grade: "-0.57%".to_string(),
            input_length: "500".to_string(),
            ..Default::default()
        };
        let curve = data.to_vertical_curve().unwrap();
        assert_eq!(
            curve.stations.pvc,
            Station {
                value: -250.,
                elevation: 1000.28
            }
        );
        assert_eq!(
            curve.stations.pvt,
            Station {
                value: 250.,
                elevation: 999.955
            }
        );
    }

    #[test]
    fn v4() {
        let data = VerticalData {
            input_method: VerticalDefinition::PVC,
            input_station: "-2+50".to_string(),
            input_elevation: "1000.28".to_string(),
            input_incoming_grade: "0.44%".to_string(),
            input_outgoing_grade: "-0.57%".to_string(),
            input_length: "500".to_string(),
            ..Default::default()
        };
        let curve = data.to_vertical_curve().unwrap();
        assert_eq!(
            curve.stations.pvi,
            Station {
                value: 0.,
                elevation: 1001.38
            }
        );
        assert_eq!(
            curve.stations.pvt,
            Station {
                value: 250.,
                elevation: 999.955
            }
        );
    }

    #[test]
    fn v5() {
        let data = VerticalData {
            input_method: VerticalDefinition::PVT,
            input_station: "2+50".to_string(),
            input_elevation: "999.955".to_string(),
            input_incoming_grade: "0.44%".to_string(),
            input_outgoing_grade: "-0.57%".to_string(),
            input_length: "500".to_string(),
            ..Default::default()
        };
        let curve = data.to_vertical_curve().unwrap();
        assert_eq!(
            curve.stations.pvi,
            Station {
                value: 0.,
                elevation: 1001.38
            }
        );
        assert_eq!(
            curve.stations.pvc,
            Station {
                value: -250.,
                elevation: 1000.28
            }
        );
    }
}
