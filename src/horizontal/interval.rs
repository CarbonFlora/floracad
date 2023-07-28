use crate::horizontal::*;
use crate::datatypes::Station;

impl HorizontalCurve {
    pub fn get_extreme(&self) -> Station {
        // let a = (self.dimensions.outgoing_grade-self.dimensions.incoming_grade)/(2.0*self.dimensions.curve_length);
        // let x = - self.dimensions.incoming_grade/(2.0*a);
        // let value = self.stations.pvc.value + x;
        // let elevation = self.stations.pvc.elevation + self.dimensions.incoming_grade*x+a*x.powi(2);

        // Station { value, elevation }
        todo!()
    }

    pub fn interval_stations(&self, station_interval: f64) -> CurveDetail {
        let mut curve_detail = CurveDetail {interval: vec![]};
        let first_station = ((self.stations.pc.value*(100.0/station_interval)/100.0).ceil())/(100.0/station_interval)*100.0;
        let mut count = 0usize;
        let mut running = Station {value: first_station, elevation: 0.0};
        
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
