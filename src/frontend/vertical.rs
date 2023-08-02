use iced::widget::{checkbox, column, text, Column};

use crate::datatypes::coerce_station_value;
use crate::frontend::*;

pub fn vertical_header_group<'a>() -> Column<'a, Message> {
    let title = text("Vertical Curves (LARGELY UNTESTED)")
        .width(Length::Fill)
        .size(50)
        .style(Color::from([0.5, 0.5, 0.5]))
        .horizontal_alignment(alignment::Horizontal::Center);
    let switch = button("Switch to Horizontal Curve").on_press(Message::SwitchCurveType);
    column![title, switch].spacing(10).width(Length::Fill)
}

pub fn vertical_input_group(vertical_data: &VerticalData) -> Column<Message> {
    let h_s = 5;

    let toggle_station = button(text(">")).on_press(Message::InputMethodToggle);
    let station_modify =
        text_input("(12+34)", &vertical_data.input_station).on_input(Message::StationModify);
    let elevation_modify =
        text_input("(152)", &vertical_data.input_elevation).on_input(Message::ElevationModify);
    let incoming_grade_modify = text_input("(0.01 = 1%)", &vertical_data.input_incoming_grade)
        .on_input(Message::IncomingGradeModify);
    let outgoing_grade_modify = text_input("(-0.02 = -2%)", &vertical_data.input_outgoing_grade)
        .on_input(Message::OutgoingGradeModify);
    let length_modify =
        text_input("(100)", &vertical_data.input_length).on_input(Message::LengthModify);
    let interval_modify = text_input("(00+25)", &vertical_data.input_station_interval)
        .on_input(Message::StationIntervalModify);
    let toggle_design_standard = button(text("A")).on_press(Message::DesignStandardToggle);
    let toggle_sight = button(text(">")).on_press(Message::SightTypeToggle);
    let design_speed =
        text_input("(65)", &vertical_data.input_design_speed).on_input(Message::DesignSpeed);
    let sustained_downgrade = checkbox(
        "Sustained Downgrade",
        vertical_data.sustained_downgrade,
        Message::SustainedDowngradeCheck,
    );

    let row_1 = row![
        text(format!("{:?} Station:", &vertical_data.input_method).as_str()),
        station_modify,
        toggle_station
    ]
    .spacing(h_s);
    let row_2 = row![
        text(format!("{:?} Elevation:", &vertical_data.input_method).as_str()),
        elevation_modify
    ]
    .spacing(h_s);
    let row_3 = row![text("Incoming Grade:"), incoming_grade_modify].spacing(h_s);
    let row_4 = row![text("Outgoing Grade:"), outgoing_grade_modify].spacing(h_s);
    let row_5 = row![text("Length:"), length_modify].spacing(h_s);
    let row_6 = row![text("Interval:"), interval_modify].spacing(h_s);
    let row_7 = row![
        text("Design Speed:"),
        design_speed,
        toggle_design_standard,
        toggle_sight
    ]
    .spacing(h_s);
    let row_8 = row![sustained_downgrade].spacing(h_s);

    column![row_1, row_2, row_3, row_4, row_5, row_6, row_7, row_8]
        .spacing(10)
        .width(Length::FillPortion(2))
        .padding(10)
}

pub fn vertical_output_group(vertical_data: &VerticalData) -> Column<Message> {
    let h_s = 5;
    let mut column = column![]
        .spacing(10)
        .width(Length::FillPortion(2))
        .padding(10);

    match vertical_data.to_vertical_curve() {
        Ok(w) => {
            let vertical_curve = text(format!("{}", w));
            column = column.push(vertical_curve);

            match w.is_compliant(
                vertical_data.input_design_standard,
                vertical_data.input_sight_type,
                calc_adjustment(vertical_data.sustained_downgrade),
            ) {
                Ok(j) => match j {
                    Some(h) => match h.0 {
                        true => {
                            column = column.push(text(format!(
                                "~ Curve Length ( {:?} - {:?} )\nCompliant as {:.2} >= {:.2}",
                                vertical_data.input_design_standard,
                                vertical_data.input_sight_type,
                                w.dimensions.curve_length,
                                h.1
                            )))
                        }
                        false => {
                            column = column.push(text(format!(
                                "~ Curve Length ( {:?} - {:?} )\nNoncompliant! as {:.2} < {:.2}",
                                vertical_data.input_design_standard,
                                vertical_data.input_sight_type,
                                w.dimensions.curve_length,
                                h.1
                            )))
                        }
                    },
                    None => {
                        column = column.push(text(format!(
                            "~ Curve Length ( {:?} - {:?} )\n{:?}",
                            vertical_data.input_design_standard,
                            vertical_data.input_sight_type,
                            vertical_data.input_design_standard
                        )))
                    }
                },
                Err(e) => {
                    column = column.push(text(format!(
                        "~ Curve Length ( {:?} - {:?} )\n{:?}",
                        vertical_data.input_design_standard, vertical_data.input_sight_type, e
                    )))
                }
            }

            column = column.push(text(format!("~ Extreme\n{}", w.get_extreme())));

            if vertical_data.input_station_interval.is_empty() {
                column = column.push(text("~ Interval Stations\nEnter an Interval."));
            } else {
                match coerce_station_value(&vertical_data.input_station_interval) {
                    Ok(t) => {
                        column = column.push(text(format!(
                            "~ Interval Stations\n{}",
                            w.interval_stations(t)
                        )));
                    }
                    Err(e) => {
                        column = column.push(text(format!("~ Interval Stations\n{}", e)));
                    }
                };
            }
        }
        Err(e) => {
            column = column.push(text(format!("{:?}", e)));
        }
    }
    column
}
