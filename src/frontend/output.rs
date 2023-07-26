use iced::widget::{column, text, Column};

use crate::{frontend::*, datatypes::coerce_station_value};

pub fn vertical_output_group(vertical_data: &VerticalData) -> Column<Message> {
    let h_s = 5;
    let mut column = column![].spacing(10).width(Length::FillPortion(2)).padding(10);
    
    match vertical_data.to_vertical_curve() {
        Ok(w) => {
            let vertical_curve = text(format!("{}", w));
            column = column.push(vertical_curve);
            
            match w.is_compliant(vertical_data.input_sight_type, w.dimensions.design_speed) {
                Some(h) => {
                    match h.0 {
                        true => column = column.push(text(format!("~ Sight Distance ({:?})\nCompliant as {} >= {}",vertical_data.input_sight_type , w.dimensions.sight_distance, h.1))),
                        false => column = column.push(text(format!("~ Sight Distance ({:?})\nNoncompliant! as {} < {}",vertical_data.input_sight_type , w.dimensions.sight_distance, h.1))),
                    }
                },
                None => column = column.push(text(format!("~ Sight Distance\nDesign speed doesn't appear in the HDM."))),
            }
            
            column = column.push(text(format!("~ Extreme\n{}", w.get_extreme())));
            
            if vertical_data.input_station_interval=="" {
                column = column.push(text(format!("~ Interval Stations\nEnter an Interval.")));
            } else {
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
            }
        },
        Err(e) => {
            let error_msg = text(format!("{:?}", e));
            
            column = column.push(error_msg);
        },
    }

    column
}