use anyhow::{anyhow, Result};

use crate::tables::get_min_sight;
use crate::vertical::*;

pub type ObstacleReturn = Result<(bool, ObstacleStation, Station, f64), Error>;

#[derive(Debug, Clone, Copy)]
pub struct VerticalStations {
    pub pvc: Station,
    pub pvi: Station,
    pub pvt: Station,
}

#[derive(Debug, Clone, Copy)]
pub struct VerticalDimensions {
    pub incoming_grade: f64,
    pub outgoing_grade: f64,
    pub curve_length: f64,
    pub external: f64,
    pub design_speed: i32,
    pub sustained_downgrade: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct VerticalCurve {
    pub dimensions: VerticalDimensions,
    pub stations: VerticalStations,
}

impl VerticalCurve {
    pub fn calc_min_curve_length(
        &self,
        min_sight: f64,
        design_standard: DesignStandard,
        sight_type: SightType,
    ) -> Result<f64> {
        let curve_length = self.dimensions.curve_length;
        let grade_break = self.dimensions.outgoing_grade - self.dimensions.incoming_grade;
        let design_speed = self.dimensions.design_speed;
        let a = grade_break.abs() * 100.0;
        if grade_break == 0.0 {
            // --
            return Ok(0.0);
        }
        let mut min_sight_adjusted = min_sight.clone();
        if a >= 2. && design_speed >= 40 && min_sight_adjusted < 10. * design_speed as f64 {
            min_sight_adjusted = 10. * design_speed as f64;
            println!("large min used.");
        }
        if a < 2. && design_speed < 40 && min_sight_adjusted < 200.0 {
            min_sight_adjusted = 200.;
            println!("small min used.");
        }
        match design_standard {
            DesignStandard::AASHTO => {
                match sight_type {
                    SightType::Stopping => {
                        if grade_break.is_sign_positive() {
                            // \/
                            let l =
                                a * min_sight_adjusted.powi(2) / (400.0 + 3.5 * min_sight_adjusted);
                            if l >= min_sight_adjusted {
                                return Ok(l);
                            }
                            let l =
                                2.0 * min_sight_adjusted - (400.0 + 3.5 * min_sight_adjusted) / a;
                            if min_sight_adjusted > l {
                                return Ok(l);
                            }

                            Err(anyhow!(format!(
                                "Failed at: {:?} - {:?} - {}",
                                design_standard, sight_type, "SAG"
                            )))
                        } else {
                            // /\
                            let l = a * min_sight_adjusted.powi(2) / 2158.0;
                            if l >= min_sight_adjusted {
                                return Ok(l);
                            }
                            let l = 2.0 * min_sight_adjusted - 2158.0 / a;
                            if min_sight_adjusted > l {
                                return Ok(l);
                            }

                            Err(anyhow!(format!(
                                "Failed at: {:?} - {:?} - {}",
                                design_standard, sight_type, "CREST"
                            )))
                        }
                    }
                    SightType::Passing => {
                        if grade_break.is_sign_positive() {
                            // \/
                            let l =
                                a * min_sight_adjusted.powi(2) / (400.0 + 3.5 * min_sight_adjusted);
                            if l >= min_sight_adjusted {
                                return Ok(l);
                            }
                            let l =
                                2.0 * min_sight_adjusted - (400.0 + 3.5 * min_sight_adjusted) / a;
                            if min_sight_adjusted > l {
                                return Ok(l);
                            }

                            Err(anyhow!(format!(
                                "Failed at: {:?} - {:?} - {}",
                                design_standard, sight_type, "SAG"
                            )))
                        } else {
                            // /\
                            let l = a * min_sight_adjusted.powi(2) / 2800.0;
                            if l >= min_sight_adjusted {
                                return Ok(l);
                            }
                            let l = 2.0 * min_sight_adjusted - 2800.0 / a;
                            if min_sight_adjusted > l {
                                return Ok(l);
                            }

                            Err(anyhow!(format!(
                                "Failed at: {:?} - {:?} - {}",
                                design_standard, sight_type, "CREST"
                            )))
                        }
                    }
                    _ => Err(anyhow!(format!(
                        "{:?} - {:?} hasn't been implimented.",
                        design_standard, sight_type
                    ))),
                }
            }
            DesignStandard::CALTRANS => {
                match sight_type {
                    SightType::Stopping => {
                        if grade_break.is_sign_positive() {
                            // \/
                            let l =
                                a * min_sight_adjusted.powi(2) / (400.0 + 3.5 * min_sight_adjusted);
                            if l >= min_sight_adjusted {
                                return Ok(l);
                            }
                            let l =
                                2.0 * min_sight_adjusted - (400.0 + 3.5 * min_sight_adjusted) / a;
                            if min_sight_adjusted > l {
                                return Ok(l);
                            }

                            Err(anyhow!(format!(
                                "Failed at: {:?} - {:?} - {}",
                                design_standard, sight_type, "SAG"
                            )))
                        } else {
                            // /\
                            let l = a * min_sight_adjusted.powi(2) / 1329.0;
                            if l >= min_sight_adjusted {
                                return Ok(l);
                            }
                            let l = 2.0 * min_sight_adjusted - 1329.0 / a;
                            if min_sight_adjusted > l {
                                return Ok(l);
                            }

                            Err(anyhow!(format!(
                                "Failed at: {:?} - {:?} - {}",
                                design_standard, sight_type, "CREST"
                            )))
                        }
                    }
                    SightType::Decision => {
                        if grade_break.is_sign_positive() {
                            // \/
                            let l =
                                a * min_sight_adjusted.powi(2) / (400.0 + 3.5 * min_sight_adjusted);
                            if l >= min_sight_adjusted {
                                return Ok(l);
                            }
                            let l =
                                2.0 * min_sight_adjusted - (400.0 + 3.5 * min_sight_adjusted) / a;
                            if min_sight_adjusted > l {
                                return Ok(l);
                            }

                            Err(anyhow!(format!(
                                "Failed at: {:?} - {:?} - {}",
                                design_standard, sight_type, "SAG"
                            )))
                        } else {
                            // /\
                            let l = a * min_sight_adjusted.powi(2) / 1329.0;
                            if l >= min_sight_adjusted {
                                return Ok(l);
                            }
                            let l = 2.0 * min_sight_adjusted - 1329.0 / a;
                            if min_sight_adjusted > l {
                                return Ok(l);
                            }

                            Err(anyhow!(format!(
                                "Failed at: {:?} - {:?} - {}",
                                design_standard, sight_type, "CREST"
                            )))
                        }
                    }
                    _ => Err(anyhow!(format!(
                        "{:?} - {:?} hasn't been implimented.",
                        design_standard, sight_type
                    ))),
                }
            }
        }
    }

    pub fn is_compliant(
        &self,
        design_standard: DesignStandard,
        sight_type: SightType,
        adjustment: f64,
    ) -> Result<(bool, f64)> {
        let min_sight = get_min_sight(self.dimensions.design_speed, design_standard, sight_type);
        match min_sight {
            Some(w) => {
                let min_curve_length =
                    self.calc_min_curve_length(w * adjustment, design_standard, sight_type)?;

                Ok((
                    (self.dimensions.curve_length >= min_curve_length),
                    min_curve_length,
                ))
            }
            None => Err(anyhow!("Design speed isn't specified in the manual.")),
        }
    }

    pub fn obstacle_compliant(&self, obstacle_detail: &ObstacleDetail) -> Vec<ObstacleReturn> {
        let mut obstacle_return = Vec::new();

        for obstacle in &obstacle_detail.interval {
            obstacle_return.push(self.within_obstacle(obstacle));
        }

        obstacle_return
    }

    fn within_obstacle(&self, obstacle: &ObstacleStation) -> ObstacleReturn {
        match self.spot_station_with_station(obstacle.0) {
            Err(e) => Err(e),
            Ok(curve_station) => {
                let delta = (curve_station.elevation - obstacle.0.elevation).abs();
                let mut within = false;

                match obstacle.1 {
                    ObstacleType::Above => {
                        if curve_station.elevation <= obstacle.0.elevation {
                            within = true;
                        }
                    }
                    ObstacleType::Below => {
                        if curve_station.elevation >= obstacle.0.elevation {
                            within = true;
                        }
                    }
                };

                Ok((within, *obstacle, curve_station, delta))
            }
        }
    }

    fn spot_station_with_station(&self, station: Station) -> Result<Station, Error> {
        if station.value >= self.stations.pvc.value && station.value <= self.stations.pvt.value {
            let distance_delta = station.value - self.stations.pvc.value;
            let a = (self.dimensions.outgoing_grade - self.dimensions.incoming_grade)
                / (2.0 * self.dimensions.curve_length);
            let elevation = self.stations.pvc.elevation
                + self.dimensions.incoming_grade * distance_delta
                + a * distance_delta.powi(2);
            return Ok(Station {
                value: station.value,
                elevation,
            });
        }

        Err(Error::ParseStation { station })
    }
}

/// Vertical Calculate Errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Obstacle station is outside the curve.
    #[error("{station} is outside the curve.")]
    ParseStation { station: Station },
}
