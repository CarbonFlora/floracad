use std::collections::HashMap;
use iced::widget::{column, text, Column};

use crate::{frontend::*, datatypes::coerce_station_value};

fn parse_vertical_data(vertical_data: &VerticalData) -> HashMap<String, String> {
    let mut raw_hash = HashMap::new();

    match vertical_data.input_method {
        VerticalDefinition::PVC => {
            raw_hash.insert("PVC-st".to_string(), vertical_data.input_station.clone());
            raw_hash.insert("PVC-elev".to_string(), vertical_data.input_elevation.clone());
        },
        VerticalDefinition::PVI => {
            raw_hash.insert("PVI-st".to_string(), vertical_data.input_station.clone());
            raw_hash.insert("PVI-elev".to_string(), vertical_data.input_elevation.clone());
        },
        VerticalDefinition::PVT => {
            raw_hash.insert("PVT-st".to_string(), vertical_data.input_station.clone());
            raw_hash.insert("PVT-elev".to_string(), vertical_data.input_elevation.clone());
        },
    };
    raw_hash.insert("inc".to_string(), vertical_data.input_incoming_grade.clone());
    raw_hash.insert("out".to_string(), vertical_data.input_outgoing_grade.clone());
    raw_hash.insert("length".to_string(), vertical_data.input_length.clone());

    raw_hash
}

pub fn vertical_output_group(vertical_data: &VerticalData) -> Column<Message> {
    let h_s = 5;
    let mut column = column![].spacing(10).width(Length::FillPortion(2)).padding(10);
    
    match vertical_data.to_vertical_curve() {
        Ok(w) => {
            let vertical_curve = text(format!("{}", w));
            column = column.push(vertical_curve);

            // let extreme_point = text(format!("{}", w.get_extreme()));
            column = column.push(text(format!("~ Extreme\n{}", w.get_extreme())));
            // column = column.push();

            match coerce_station_value(vertical_data.input_station_interval.clone()) {
                Ok(t) => {
                    let vertical_detail = text(format!("{}", w.interval_stations(t)));
                    column = column.push(vertical_detail);
                },
                Err(e) => {
                    let error_msg = text(format!("{:?}", e));
            
                    column = column.push(error_msg);
                }
            };
        },
        Err(e) => {
            let error_msg = text(format!("{:?}", e));
            
            column = column.push(error_msg);
        },
    }

    column
}