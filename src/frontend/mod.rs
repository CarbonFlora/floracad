use iced::alignment::{self};
use iced::theme::Theme;
use iced::widget::{button, column, container, row, scrollable, text_input};
use iced::{Application, Element};
use iced::{Color, Command, Length};
use once_cell::sync::Lazy;

pub mod vertical;
pub mod horizontal;

use crate::frontend::vertical::*;
use crate::frontend::horizontal::*;
use crate::vertical::*;
use crate::horizontal::*;
use crate::datatypes::*;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

#[derive(Debug)]
pub enum CurveSolver {
    Vertical(VerticalData),
    Horizontal(HorizontalData),
}


#[derive(Debug, Clone)]
pub enum Message {
    // Vertical
    InputMethodToggle,
    StationModify(String),
    ElevationModify(String),
    IncomingGradeModify(String),
    OutgoingGradeModify(String),
    LengthModify(String),
    StationIntervalModify(String),
    DesignStandardToggle,
    SightTypeToggle,
    DesignSpeed(String),
    // Horizontal
    StationMethodToggle,
    BuildMethodToggle,
    RadiusModify(String),
    CurveAngleModify(String),
}

impl Application for CurveSolver {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (CurveSolver, Command<Message>) {
        (
            CurveSolver::Vertical(
                VerticalData { 
                    input_method: VerticalDefinition::PVI, 
                    input_station: "".to_string(), 
                    input_elevation: "".to_string(), 
                    input_incoming_grade: "".to_string(), 
                    input_outgoing_grade: "".to_string(), 
                    input_length: "".to_string(), 
                    input_station_interval: "".to_string(), 
                    input_sight_type: crate::datatypes::SightType::Stopping, 
                    input_design_speed: "".to_string(),
                    input_design_standard: DesignStandard::AASHTO
                }),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Zi's Curve Solver")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            CurveSolver::Vertical(vertical_data) => {
                let command = match message {
                    Message::InputMethodToggle => {
                        vertical_data.input_method = vertical_data.input_method.next();
                        Command::none()
                    },
                    Message::StationModify(raw_input) => {
                        vertical_data.input_station = raw_input;
                        Command::none()
                    },
                    Message::ElevationModify(raw_input) => {
                        vertical_data.input_elevation = raw_input;
                        Command::none()
                    },
                    Message::IncomingGradeModify(raw_input) => {
                        vertical_data.input_incoming_grade = raw_input;
                        Command::none()
                    },
                    Message::OutgoingGradeModify(raw_input) => {
                        vertical_data.input_outgoing_grade = raw_input;
                        Command::none()
                    },
                    Message::LengthModify(raw_input) => {
                        vertical_data.input_length = raw_input;
                        Command::none()
                    },
                    Message::StationIntervalModify(raw_input) => {
                        vertical_data.input_station_interval = raw_input;
                        Command::none()
                    },
                    Message::DesignStandardToggle => {
                        vertical_data.input_design_standard = vertical_data.input_design_standard.next();
                        Command::none()
                    },
                    Message::SightTypeToggle => {
                        vertical_data.input_sight_type = vertical_data.input_sight_type.next();
                        Command::none()
                    },
                    Message::DesignSpeed(raw_input) => {
                        vertical_data.input_design_speed = raw_input;
                        Command::none()
                    },
                    _ => {
                        Command::none()
                    }
                };
                Command::batch(vec![command])
            },
            CurveSolver::Horizontal(horizontal_data) => {
                let command = match message {
                    _ => {
                        Command::none()
                    }
                };
                Command::batch(vec![command])
            },
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            CurveSolver::Vertical(vertical_data) => {
                let title = vertical_header_group();
                let body = row![vertical_input_group(vertical_data), vertical_output_group(vertical_data)];

                scrollable(
                    container(column![title, body].spacing(10))
                        .width(Length::Fill)
                        .padding(40)
                        .center_x(),
                )
                .into()
            },
            CurveSolver::Horizontal(horizontal_data) => {
                let title = horizontal_header_group();
                let body = row![horizontal_input_group(horizontal_data), horizontal_output_group(horizontal_data)];

                scrollable(
                    container(column![title, body].spacing(10))
                        .width(Length::Fill)
                        .padding(40)
                        .center_x(),
                )
                .into()
            }
        }
    }
}
