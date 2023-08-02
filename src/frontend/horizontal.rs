use iced::widget::{column, text, Column, checkbox};

use crate::frontend::*;

pub fn horizontal_header_group<'a>() -> Column<'a, Message> {
    let title = text("Horizontal Curves (LARGELY UNTESTED)")
        .width(Length::Fill)
        .size(50)
        .style(Color::from([0.5, 0.5, 0.5]))
        .horizontal_alignment(alignment::Horizontal::Center);
    let switch = button("Switch to Vertical Curve")
        .on_press(Message::SwitchCurveType);
    column![title, switch].spacing(10).width(Length::Fill)
}

pub fn horizontal_input_group(horizontal_data: &HorizontalData) -> Column<Message> {
    let h_s = 5;
    
    let toggle_station = button(text(">"))
        .on_press(Message::StationMethodToggle);
    let station_modify = text_input(format!("(12+34)").as_str(), &horizontal_data.input_station)
        .on_input(Message::StationModify);
    let toggle_build = button(text("#"))
        .on_press(Message::BuildMethodToggle);
    let radius_modify = text_input("(100)", &horizontal_data.input_radius)
        .on_input(Message::RadiusModify);
    let curve_angle_modify = text_input("(60d13\'42\")", &horizontal_data.input_curve_angle)
        .on_input(Message::CurveAngleModify);
    // let length_modify = text_input("(100)", &horizontal_data.input_length)
    //     .on_input(Message::LengthModify);
    let interval_modify = text_input("(00+25)", &horizontal_data.input_station_interval)
        .on_input(Message::StationIntervalModify);
    let toggle_design_standard = button(text("A"))
        .on_press(Message::DesignStandardToggle);
    let toggle_sight = button(text(">"))
        .on_press(Message::SightTypeToggle);
    let design_speed = text_input("(65)", &horizontal_data.input_design_speed)
        .on_input(Message::DesignSpeed);
    let m = text_input("(100)", &horizontal_data.input_m)
        .on_input(Message::MModify);
    let sustained_downgrade = checkbox("Sustained Downgrade", horizontal_data.sustained_downgrade, Message::SustainedDowngradeCheck);
    
    let row_1 = row![text(format!("{:?} Station:", &horizontal_data.input_station_method).as_str()), station_modify, toggle_station, toggle_build].spacing(h_s);
    let mut row_stack_2 = column![].spacing(h_s);
    match horizontal_data.input_build_method {
        HorizontalBuildDefinition::RadiusCurveAngle => {
            row_stack_2 = row_stack_2.push(row![text("Radius:"), radius_modify].spacing(h_s));
            row_stack_2 = row_stack_2.push(row![text("Curve Angle:"), curve_angle_modify].spacing(h_s));
        },
        _ => (),
    }

    let row_6 = row![text("Interval:"), interval_modify].spacing(h_s);
    let row_7 = row![text("Design Speed:"), design_speed, toggle_design_standard, toggle_sight].spacing(h_s);
    let row_8 = row![text("Clear Distance from C/L:"), m].spacing(h_s);
    let row_9 = row![sustained_downgrade].spacing(h_s);

    column![row_1, row_stack_2, row_6, row_7, row_8, row_9]
        .spacing(10)
        .width(Length::FillPortion(2))
        .padding(10)
}

pub fn horizontal_output_group(horizontal_data: &HorizontalData) -> Column<Message> {
    let h_s = 5;
    let mut column = column![].spacing(10).width(Length::FillPortion(2)).padding(10);
    
    match horizontal_data.to_horizontal_curve() {
        Ok(w) => {
            let horizontal_curve = text(format!("{}", w));
            column = column.push(horizontal_curve);
            
            match w.is_compliant(horizontal_data.input_design_standard, horizontal_data.input_sight_type, calc_adjustment(horizontal_data.sustained_downgrade)) {
                Ok(j) => {
                    match j {
                        Some(h) => {
                            match h.0 {
                                true => column = column.push(text(format!("~ Sight Distance ( {:?} - {:?} )\nCompliant as {:.2} >= {:.2}",horizontal_data.input_design_standard, horizontal_data.input_sight_type, w.dimensions.sight_distance, h.1))),
                                false => column = column.push(text(format!("~ Sight Distance ( {:?} - {:?} )\nNoncompliant! as {:.2} < {:.2}",horizontal_data.input_design_standard, horizontal_data.input_sight_type, w.dimensions.sight_distance, h.1))),
                            }
                        },
                        None => column = column.push(text(format!("~ Sight Distance ( {:?} - {:?} )\n{:?}",horizontal_data.input_design_standard, horizontal_data.input_sight_type, horizontal_data.input_design_standard))),
                    }
                },
                Err(e) => column = column.push(text(format!("~ Sight Distance ( {:?} - {:?} )\n{:?}",horizontal_data.input_design_standard, horizontal_data.input_sight_type, e))),
            }
            
            // column = column.push(text(format!("~ Extreme\n{}", w.get_extreme())));
            
            if horizontal_data.input_station_interval=="" {
                column = column.push(text(format!("~ Interval Stations\nEnter an Interval.")));
            } else {
                match coerce_station_value(&horizontal_data.input_station_interval) {
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