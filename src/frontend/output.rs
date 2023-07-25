use std::collections::HashMap;
use iced::widget::{column, text, Column};

use crate::frontend::*;

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
    
    match vertical_data.to_vertical_curve() {
        Ok(w) => {
            let curve_details = text(format!("{:#?}", w));

            column![curve_details]
                .spacing(10)
                .width(Length::FillPortion(2))
                .padding(10)
        },
        Err(e) => {
            let error_msg = text(format!("{:?}", e));
            
            column![error_msg]
                .spacing(10)
                .width(Length::FillPortion(2))
                .padding(10)
        },
    }
}