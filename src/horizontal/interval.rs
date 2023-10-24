use crate::datatypes::Station;
use crate::horizontal::*;

impl HorizontalCurve {
    pub fn interval_stations(&self, station_interval: f64) -> CurveDetail {
        let mut curve_detail = CurveDetail { interval: vec![] };
        let first_station = ((self.stations.pc.value * (100.0 / station_interval) / 100.0).ceil())
            / (100.0 / station_interval)
            * 100.0;
        let length1 = first_station - self.stations.pc.value;
        let delta1_radians =
            length1 / self.dimensions.curve_length * self.dimensions.curve_angle.radians;
        let deflection1_radians = delta1_radians / 2.;
        let chord1 = 2. * self.dimensions.radius * (deflection1_radians).sin();

        let mut count = 0usize;
        let mut running = Station {
            value: first_station,
            deflection: Some(Angle {
                radians: deflection1_radians,
            }),
            chord: Some(chord1),
            ..Default::default()
        };

        loop {
            curve_detail.interval.push(running);

            let running_deflection = running.deflection.unwrap().radians;
            running.deflection = Some(Angle {
                radians: running_deflection + self.dimensions.curve_length_100.radians / 2.,
            });

            running.chord = Some(
                2. * self.dimensions.radius * (self.dimensions.curve_length_100.radians / 2.).sin(),
            );

            running.value += station_interval;

            count += 1;
            if running.value >= self.stations.pt.value || count >= 100 {
                break;
            }
        }

        //handle pt here because too lazy to do it properly.
        let length2 = self.stations.pt.value - (running.value - station_interval);
        let delta2_radians =
            length2 / self.dimensions.curve_length * self.dimensions.curve_angle.radians;
        let deflection2_radians = delta2_radians / 2.;
        let chord2 = 2. * self.dimensions.radius * (deflection2_radians).sin();
        curve_detail.interval.push(Station {
            value: self.stations.pt.value,
            deflection: Some(Angle {
                radians: deflection2_radians + running.deflection.unwrap().radians,
            }),
            chord: Some(chord2),
            ..Default::default()
        });

        curve_detail
    }
}
