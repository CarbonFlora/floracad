use iced::widget::{checkbox, column, text, Column, Row};

use crate::{frontend::*, horizontal::calculate::HorizontalCurve};

pub fn horizontal_header_group<'a>() -> Column<'a, Message> {
    let title = text("Horizontal Curves (Pre-Release)")
        .width(Length::Fill)
        .size(50)
        .style(Color::from([0.5, 0.5, 0.5]))
        .horizontal_alignment(alignment::Horizontal::Center);
    let switch = button("Switch to Vertical Curve").on_press(Message::SwitchCurveType);
    column![title, switch].spacing(10).width(Length::Fill)
}

impl HorizontalData {
    pub fn horizontal_input_group(&self) -> Column<Message> {
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
                text(format!("{:?} STA:", &self.input_station_method)),
                text_input("Station (12+34)", &self.input_station).on_input(Message::StationModify),
                button(stext('S')).on_press(Message::StationMethodToggle),
                button(cycle_icon()).on_press(Message::BuildMethodToggle),
            ]
            .spacing(H_S),
            match self.input_build_method {
                HorizontalBuildDefinition::RadiusCurveAngle => self.row_radius_curve_angle(),
                HorizontalBuildDefinition::RadiusTangent => self.row_radius_tangent(),
            }
            .spacing(H_S),
        ]
        .spacing(H_S)
    }

    fn row_radius_curve_angle(&self) -> Row<'_, Message> {
        row![
            text("Radius:"),
            text_input("(100)", &self.input_radius).on_input(Message::RadiusModify),
            text("Curve Angle:"),
            text_input("(60d13\'42\")", &self.input_curve_angle)
                .on_input(Message::CurveAngleModify)
        ]
    }

    fn row_radius_tangent(&self) -> Row<'_, Message> {
        row![
            text("Radius:"),
            text_input("(100)", &self.input_radius).on_input(Message::RadiusModify),
            text("Tangent:"),
            text_input("(123)", &self.input_tangent).on_input(Message::TangentModify)
        ]
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
                text("Clear Distance:"),
                text_input("(234)", &self.input_m).on_input(Message::MModify),
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
        ]
        .spacing(H_S)
    }

    pub fn horizontal_output_group(&self) -> Column<Message> {
        let h_s = 5;
        let mut column = column![]
            .spacing(10)
            .width(Length::FillPortion(2))
            .padding(10);

        match self.to_horizontal_curve() {
            Err(e) => {
                column = column.push(row![exclam_icon(), text(format!(" {}", e))]);
            }
            Ok(w) => {
                column = column
                    .push(self.curve_details_block(&w))
                    .push(self.major_stations_block(&w))
                    .push(self.validation_block(&w))
                    .push(self.interval_block(&w));
            }
        }
        column
    }

    fn curve_details_block(&self, w: &HorizontalCurve) -> Column<'_, Message> {
        column![subtitle("Curve Details"), text(format!("{}", w.dimensions)),]
    }

    fn major_stations_block(&self, w: &HorizontalCurve) -> Column<'_, Message> {
        column![subtitle("Major Stations"), text(format!("{}", w.stations)),]
    }

    fn validation_block(&self, w: &HorizontalCurve) -> Column<'_, Message> {
        let mut validation_column = column![subtitle(&format!(
            "Sight Distance ( {:?} - {:?} )",
            self.input_design_standard, self.input_sight_type
        ))];
        if self.input_design_speed.is_empty() || self.input_m.is_empty() {
            validation_column = validation_column.push(row![
                notification_icon(),
                text(" No speed or clear distance given.")
            ]);
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
                            text(format!(" {:.2} > {:.2}", w.dimensions.sight_distance, j.1))
                        ]);
                    } else {
                        validation_column = validation_column.push(row![
                            exclam_icon(),
                            text(format!(" {:.2} < {:.2}", w.dimensions.sight_distance, j.1))
                        ]);
                    }
                }
            }
        }

        validation_column
    }

    fn interval_block(&self, w: &HorizontalCurve) -> Column<'_, Message> {
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
