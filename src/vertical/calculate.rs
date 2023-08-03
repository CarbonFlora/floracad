use anyhow::{anyhow, Result};

use crate::tables::get_min_sight;
use crate::vertical::*;

type ObstacleReturn = (bool, Option<ObstacleStation>, Option<Station>, f64);

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
        let a = grade_break.abs() * 100.0;
        if grade_break == 0.0 {
            // --
            return Ok(0.0);
        }

        match design_standard {
            DesignStandard::AASHTO => {
                match sight_type {
                    SightType::Stopping => {
                        if grade_break.is_sign_positive() {
                            // \/
                            let l = a * min_sight.powi(2) / (400.0 + 3.5 * min_sight);
                            if l >= min_sight {
                                return Ok(l);
                            }
                            let l = 2.0 * min_sight - (400.0 + 3.5 * min_sight) / a;
                            if min_sight > l {
                                return Ok(l);
                            }

                            Err(anyhow!(format!(
                                "Failed at: {:?} - {:?} - {}",
                                design_standard, sight_type, "SAG"
                            )))
                        } else {
                            // /\
                            let l = a * min_sight.powi(2) / 2158.0;
                            if l >= min_sight {
                                return Ok(l);
                            }
                            let l = 2.0 * min_sight - 2158.0 / a;
                            if min_sight > l {
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
                            let l = a * min_sight.powi(2) / (400.0 + 3.5 * min_sight);
                            if l >= min_sight {
                                return Ok(l);
                            }
                            let l = 2.0 * min_sight - (400.0 + 3.5 * min_sight) / a;
                            if min_sight > l {
                                return Ok(l);
                            }

                            Err(anyhow!(format!(
                                "Failed at: {:?} - {:?} - {}",
                                design_standard, sight_type, "SAG"
                            )))
                        } else {
                            // /\
                            let l = a * min_sight.powi(2) / 2800.0;
                            if l >= min_sight {
                                return Ok(l);
                            }
                            let l = 2.0 * min_sight - 2800.0 / a;
                            if min_sight > l {
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
                            let l = a * min_sight.powi(2) / (400.0 + 3.5 * min_sight);
                            if l >= min_sight {
                                return Ok(l);
                            }
                            let l = 2.0 * min_sight - (400.0 + 3.5 * min_sight) / a;
                            if min_sight > l {
                                return Ok(l);
                            }

                            Err(anyhow!(format!(
                                "Failed at: {:?} - {:?} - {}",
                                design_standard, sight_type, "SAG"
                            )))
                        } else {
                            // /\
                            let l = a * min_sight.powi(2) / 1329.0;
                            if l >= min_sight {
                                return Ok(l);
                            }
                            let l = 2.0 * min_sight - 1329.0 / a;
                            if min_sight > l {
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
                            let l = a * min_sight.powi(2) / (400.0 + 3.5 * min_sight);
                            if l >= min_sight {
                                return Ok(l);
                            }
                            let l = 2.0 * min_sight - (400.0 + 3.5 * min_sight) / a;
                            if min_sight > l {
                                return Ok(l);
                            }

                            Err(anyhow!(format!(
                                "Failed at: {:?} - {:?} - {}",
                                design_standard, sight_type, "SAG"
                            )))
                        } else {
                            // /\
                            let l = a * min_sight.powi(2) / 1329.0;
                            if l >= min_sight {
                                return Ok(l);
                            }
                            let l = 2.0 * min_sight - 1329.0 / a;
                            if min_sight > l {
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
    ) -> Result<Option<(bool, f64)>> {
        let min_sight = get_min_sight(self.dimensions.design_speed, design_standard, sight_type);
        match min_sight {
            Some(w) => {
                let min_curve_length =
                    self.calc_min_curve_length(w * adjustment, design_standard, sight_type)?;

                Ok(Some((
                    (self.dimensions.curve_length >= min_curve_length),
                    min_curve_length,
                )))
            }
            None => Err(anyhow!("Design speed isn't specified in the manual.")),
        }
    }

    pub fn within_obstacles(&self, obstacle_detail: &ObstacleDetail) -> Result<ObstacleReturn> {
        for obstacle in &obstacle_detail.interval {
            let curve_station = self.spot_station_with_station(obstacle.0)?;
            let delta = (curve_station.elevation - obstacle.0.elevation).abs();

            match obstacle.1 {
                ObstacleType::Above => {
                    if curve_station.elevation >= obstacle.0.elevation {
                        return Ok((false, Some(*obstacle), Some(curve_station), delta));
                    }
                }
                ObstacleType::Below => {
                    if curve_station.elevation <= obstacle.0.elevation {
                        return Ok((false, Some(*obstacle), Some(curve_station), delta));
                    }
                }
            };
        }

        Ok((true, None, None, 0.))
    }

    fn spot_station_with_station(&self, station: Station) -> Result<Station> {
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

        Err(anyhow!("{} is outside the curve.", station))
    }
}
