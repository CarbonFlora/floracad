use iced::widget::{checkbox, column, text, Column};

use crate::datatypes::coerce_station_value;
use crate::frontend::*;
use crate::vertical::calculate::{ObstacleReturn, VerticalCurve};

pub fn vertical_header_group<'a>() -> Column<'a, Message> {
    let title = text("Vertical Curves (Pre-Release)")
        .width(Length::Fill)
        .size(50)
        .style(Color::from([0.5, 0.5, 0.5]))
        .horizontal_alignment(alignment::Horizontal::Center);
    let switch = button("Switch to Horizontal Curve").on_press(Message::SwitchCurveType);
    column![title, switch].spacing(10).width(Length::Fill)
}

impl VerticalData {
    pub fn vertical_input_group(&self) -> Column<Message> {
        let h_s = 5;

        let toggle_station = button(stext('S')).on_press(Message::InputMethodToggle);
        let station_modify =
            text_input("(12+34)", &self.input_station).on_input(Message::StationModify);
        let elevation_modify =
            text_input("(152)", &self.input_elevation).on_input(Message::ElevationModify);
        let incoming_grade_modify = text_input("(0.01 = 1%)", &self.input_incoming_grade)
            .on_input(Message::IncomingGradeModify);
        let outgoing_grade_modify = text_input("(-0.02 = -2%)", &self.input_outgoing_grade)
            .on_input(Message::OutgoingGradeModify);
        let length_modify = text_input("(100)", &self.input_length).on_input(Message::LengthModify);
        let interval_modify = text_input("(00+25)", &self.input_station_interval)
            .on_input(Message::StationIntervalModify);
        let toggle_design_standard = button(stext('A')).on_press(Message::DesignStandardToggle);
        let toggle_sight = button(cycle_icon()).on_press(Message::SightTypeToggle);
        let design_speed =
            text_input("(65)", &self.input_design_speed).on_input(Message::DesignSpeed);
        let sustained_downgrade = checkbox(
            "Sustained Downgrade",
            self.sustained_downgrade,
            Message::SustainedDowngradeCheck,
        );
        let obstacle_station =
            text_input("Station", &self.input_obstacle_station).on_input(Message::ObstacleStation);
        let obstacle_elevation = text_input("Elevation", &self.input_obstacle_elevation)
            .on_input(Message::ObstacleElevation);
        let obstacle_type_toggle = button(Self::obs_type_toggle_arrow(self.input_obstacle_type))
            .on_press(Message::ObstacleTypeToggle);
        let obstacle_add = button(stext('+')).on_press(Message::AddObstacle);
        let obstacle_remove = button(stext('-')).on_press(Message::RemoveObstacle);

        let row_1 = row![
            text(format!("{:?} Station:", &self.input_method).as_str()),
            station_modify,
            toggle_station
        ]
        .spacing(h_s);
        let row_2 = row![
            text(format!("{:?} Elevation:", &self.input_method).as_str()),
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
        let row_9 = row![
            text("Obstacles:"),
            obstacle_station,
            obstacle_elevation,
            obstacle_type_toggle,
            obstacle_add,
            obstacle_remove
        ]
        .spacing(h_s);
        let row_10 = row![text(format!("{}", self.obstacles))].spacing(h_s);

        column![row_1, row_2, row_3, row_4, row_5, row_6, row_7, row_8, row_9, row_10]
            .spacing(10)
            .width(Length::FillPortion(2))
            .padding(10)
    }

    fn obs_type_toggle_arrow(arrow_type: ObstacleType) -> Text<'static> {
        match arrow_type {
            ObstacleType::Above => up_arrow_icon(),
            ObstacleType::Below => down_arrow_icon(),
        }
    }

    pub fn vertical_output_group(&self) -> Column<Message> {
        let h_s = 5;
        let mut column = column![]
            .spacing(10)
            .width(Length::FillPortion(2))
            .padding(10);

        match self.to_vertical_curve() {
            Ok(w) => {
                column = column
                    .push(self.curve_details_block(&w))
                    .push(self.major_stations_block(&w))
                    .push(self.extreme_block(&w))
                    .push(self.validation_block(&w))
                    .push(self.obstacle_block(&w))
                    .push(self.interval_block(&w));
            }
            Err(e) => {
                column = column.push(row![exclam_icon(), text(format!(" {}", e))]);
            }
        }
        column
    }

    fn curve_details_block(&self, w: &VerticalCurve) -> Column<'_, Message> {
        column![subtitle("Curve Details"), text(format!("{}", w.dimensions)),]
    }

    fn major_stations_block(&self, w: &VerticalCurve) -> Column<'_, Message> {
        column![subtitle("Major Stations"), text(format!("{}", w.stations)),]
    }

    fn extreme_block(&self, w: &VerticalCurve) -> Column<'_, Message> {
        column![subtitle("Extremity"), text(format!("{}", w.get_extreme()))]
    }

    fn obstacle_block(&self, w: &VerticalCurve) -> Column<'_, Message> {
        let mut obstacle_column = column![subtitle("Obstacle Validation")];
        let obstacle_calc: Vec<ObstacleReturn> = w.obstacle_compliant(&self.obstacles);
        for obstacle_return in obstacle_calc {
            match obstacle_return {
                Err(e) => {
                    obstacle_column =
                        obstacle_column.push(row![exclam_icon(), text(format!(" {}", e))]);
                }
                Ok(w) => {
                    if w.0 {
                        obstacle_column = obstacle_column.push(row![
                            good_check_icon(),
                            text(format!(
                                "Obstacle: {} {:?} Actual: {} Delta: {:2}",
                                w.1 .0, w.1 .1, w.2, w.3
                            ))
                        ]);
                    } else {
                        obstacle_column = obstacle_column.push(row![
                            exclam_icon(),
                            text(format!(
                                "Obstacle: {} {:?} Actual: {} Delta: {:2}",
                                w.1 .0, w.1 .1, w.2, w.3
                            ))
                        ]);
                    }
                }
            };
        }

        obstacle_column
    }

    fn validation_block(&self, w: &VerticalCurve) -> Column<'_, Message> {
        let mut validation_column = column![subtitle(&format!(
            "Curve Length ( {:?} - {:?} )",
            self.input_design_standard, self.input_sight_type
        ))];

        match w.is_compliant(
            self.input_design_standard,
            self.input_sight_type,
            calc_adjustment(self.sustained_downgrade),
        ) {
            Err(e) => {
                validation_column =
                    validation_column.push(row![exclam_icon(), text(format!(" {}", e))]);
            }
            Ok(j) => {
                if j.0 {
                    validation_column = validation_column.push(row![
                        good_check_icon(),
                        text(format!(" {:.2} > {:.2}", w.dimensions.curve_length, j.1))
                    ]);
                } else {
                    validation_column = validation_column.push(row![
                        exclam_icon(),
                        text(format!(" {:.2} < {:.2}", w.dimensions.curve_length, j.1))
                    ]);
                }
            }
        }
        validation_column
    }

    fn interval_block(&self, w: &VerticalCurve) -> Column<'_, Message> {
        let mut interval_column = column![subtitle("Interval Stations")];

        match coerce_station_value(&self.input_station_interval) {
            Err(e) => {
                interval_column =
                    interval_column.push(row![exclam_icon(), text(format!(" {}", e))]);
            }
            Ok(t) => {
                interval_column = interval_column.push(text(format!("{}", w.interval_stations(t))));
            }
        };

        interval_column
    }
}
