use crate::datatypes::Station;
use crate::horizontal::*;

impl HorizontalCurve {
    pub fn interval_stations(&self, station_interval: f64) -> CurveDetail {
        let mut curve_detail = CurveDetail { interval: vec![] };
        let first_station = ((self.stations.pc.value * (100.0 / station_interval) / 100.0).ceil())
            / (100.0 / station_interval)
            * 100.0;
        let mut count = 0usize;
        let mut running = Station {
            value: first_station,
            elevation: 0.0,
        };

        loop {
            curve_detail.interval.push(running);

            running.value += station_interval;

            count += 1;
            if running.value >= self.stations.pt.value || count >= 100 {
                break;
            }
        }

        curve_detail
    }
}
