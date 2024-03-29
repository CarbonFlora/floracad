use crate::datatypes::Station;
use crate::vertical::*;

impl VerticalCurve {
    pub fn get_extreme(&self) -> Station {
        let a = (self.dimensions.outgoing_grade - self.dimensions.incoming_grade)
            / (2.0 * self.dimensions.curve_length);
        let x = -self.dimensions.incoming_grade / (2.0 * a);
        let value = self.stations.pvc.value + x;
        let elevation = self.stations.pvc.elevation.unwrap()
            + self.dimensions.incoming_grade * x
            + a * x.powi(2);

        Station {
            value,
            elevation: Some(elevation),
            ..Default::default()
        }
    }

    pub fn interval_stations(&self, station_interval: f64) -> CurveDetail {
        let mut curve_detail = CurveDetail { interval: vec![] };
        let pvc_elevation = self.stations.pvc.elevation;
        let a = (self.dimensions.outgoing_grade - self.dimensions.incoming_grade)
            / (2.0 * self.dimensions.curve_length);
        let first_station = ((self.stations.pvc.value * (100.0 / station_interval) / 100.0).ceil())
            / (100.0 / station_interval)
            * 100.0;
        let mut count = 0usize;
        let mut x = first_station - self.stations.pvc.value;
        let mut running = Station {
            value: first_station,
            elevation: Some(
                pvc_elevation.unwrap() + self.dimensions.incoming_grade * x + a * x.powi(2),
            ),
            ..Default::default()
        };

        loop {
            curve_detail.interval.push(running);

            running.value += station_interval;
            x = running.value - first_station;
            running.elevation =
                Some(pvc_elevation.unwrap() + self.dimensions.incoming_grade * x + a * x.powi(2));
            count += 1;
            if running.value >= self.stations.pvt.value || count >= 100 {
                break;
            }
        }

        curve_detail
    }
}
