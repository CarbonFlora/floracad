use iced::widget::{column, text, Column};

use crate::{frontend::*, datatypes::coerce_station_value};

pub fn vertical_output_group(vertical_data: &VerticalData) -> Column<Message> {
    let h_s = 5;
    let mut column = column![].spacing(10).width(Length::FillPortion(2)).padding(10);
    
    match vertical_data.to_vertical_curve() {
        Ok(w) => {
            let vertical_curve = text(format!("{}", w));
            column = column.push(vertical_curve);
            
            match w.is_compliant(vertical_data.input_design_standard, vertical_data.input_sight_type) {
                Ok(j) => {
                    match j {
                        Some(h) => {
                            match h.0 {
                                true => column = column.push(text(format!("~ Sight Distance ( {:?} - {:?} )\nCompliant as {:.2} >= {:.2}",vertical_data.input_design_standard, vertical_data.input_sight_type, w.dimensions.curve_length, h.1))),
                                false => column = column.push(text(format!("~ Sight Distance ( {:?} - {:?} )\nNoncompliant! as {:.2} < {:.2}",vertical_data.input_design_standard, vertical_data.input_sight_type, w.dimensions.curve_length, h.1))),
                            }
                        },
                        None => column = column.push(text(format!("~ Sight Distance ( {:?} - {:?} )\n{:?}",vertical_data.input_design_standard, vertical_data.input_sight_type, vertical_data.input_design_standard))),
                    }
                },
                Err(e) => column = column.push(text(format!("~ Sight Distance ( {:?} - {:?} )\n{:?}",vertical_data.input_design_standard, vertical_data.input_sight_type, e))),
            }
            
            column = column.push(text(format!("~ Extreme\n{}", w.get_extreme())));
            
            if vertical_data.input_station_interval=="" {
                column = column.push(text(format!("~ Interval Stations\nEnter an Interval.")));
            } else {
                match coerce_station_value(&vertical_data.input_station_interval) {
                    Ok(t) => {    
                        column = column.push(text(format!("~ Interval Stations\n{}",w.interval_stations(t))));
                    },
                    Err(e) => {
                        column = column.push(text(format!("~ Interval Stations\n{}",e)));
                    }
                };
            }
        },
        Err(e) => {
            column = column.push(text(format!("{:?}", e)));
        },
    }

    column
}