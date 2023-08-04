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
        let mut column = column![]
            .spacing(10)
            .width(Length::FillPortion(2))
            .padding(10);

        column = column
            .push(self.mandatory_block())
            .push(self.optional_block());

        column
    }

    fn mandatory_block(&self) -> Column<'_, Message> {
        column![
            subtitle("Inputs"),
            row![
                text(format!("{:?} STA:", &self.input_method)),
                text_input("Station (12+34)", &self.input_station).on_input(Message::StationModify),
                text("ELEV:"),
                text_input("Elevation (5678)", &self.input_elevation)
                    .on_input(Message::ElevationModify),
                button(stext('S')).on_press(Message::InputMethodToggle)
            ]
            .spacing(H_S),
            row![
                text("Grade In:"),
                text_input("(0.01 = 1%)", &self.input_incoming_grade)
                    .on_input(Message::IncomingGradeModify),
                text("Grade Out:"),
                text_input("(-0.02 = -2%)", &self.input_outgoing_grade)
                    .on_input(Message::OutgoingGradeModify),
            ]
            .spacing(H_S),
            row![
                text("Length:"),
                text_input("(100)", &self.input_length).on_input(Message::LengthModify)
            ]
            .spacing(H_S)
        ]
        .spacing(H_S)
    }

    fn optional_block(&self) -> Column<'_, Message> {
        column![
            subtitle("Additional Details"),
            row![
                text("Interval:"),
                text_input("(00+25)", &self.input_station_interval)
                    .on_input(Message::StationIntervalModify),
            ]
            .spacing(H_S),
            row![
                text("Design Speed:"),
                text_input("(65)", &self.input_design_speed).on_input(Message::DesignSpeed),
                button(stext('A')).on_press(Message::DesignStandardToggle),
                button(cycle_icon()).on_press(Message::SightTypeToggle),
            ]
            .spacing(H_S),
            checkbox(
                "Sustained Downgrade",
                self.sustained_downgrade,
                Message::SustainedDowngradeCheck,
            )
            .spacing(H_S),
            row![
                text("Obstacles:"),
                text_input("STA:", &self.input_obstacle_station).on_input(Message::ObstacleStation),
                text_input("ELEV:", &self.input_obstacle_elevation)
                    .on_input(Message::ObstacleElevation),
                button(Self::obs_type_toggle_arrow(self.input_obstacle_type))
                    .on_press(Message::ObstacleTypeToggle),
                button(stext('+')).on_press(Message::AddObstacle),
                button(stext('-')).on_press(Message::RemoveObstacle)
            ]
            .spacing(H_S)
        ]
        .spacing(H_S)
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

        if self.obstacles.interval.is_empty() {
            obstacle_column =
                obstacle_column.push(row![notification_icon(), text(" No obstacles given.")]);
        } else {
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
        }

        obstacle_column
    }

    fn validation_block(&self, w: &VerticalCurve) -> Column<'_, Message> {
        let mut validation_column = column![subtitle(&format!(
            "Curve Length ( {:?} - {:?} )",
            self.input_design_standard, self.input_sight_type
        ))];

        if self.input_design_speed.is_empty() {
            validation_column =
                validation_column.push(row![notification_icon(), text(" No speed given.")]);
        } else {
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
        }

        validation_column
    }

    fn interval_block(&self, w: &VerticalCurve) -> Column<'_, Message> {
        let mut interval_column = column![subtitle("Interval Stations")];

        if self.input_station_interval.is_empty() {
            interval_column =
                interval_column.push(row![notification_icon(), text(" No interval given.")]);
        } else {
            match coerce_station_value(&self.input_station_interval) {
                Err(e) => {
                    interval_column =
                        interval_column.push(row![exclam_icon(), text(format!(" {}", e))]);
                }
                Ok(t) => {
                    interval_column =
                        interval_column.push(text(format!("{}", w.interval_stations(t))));
                }
            };
        }

        interval_column
    }
}
