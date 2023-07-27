use iced::alignment::{self};
use iced::theme::Theme;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Column};
use iced::{Application, Element};
use iced::{Color, Command, Length};
use once_cell::sync::Lazy;

pub mod output;

use crate::frontend::output::vertical_output_group;
use crate::vertical::*;
use crate::datatypes::*;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

#[derive(Debug)]
pub enum CurveSolver {
    Vertical(VerticalData),
}


#[derive(Debug, Clone)]
pub enum Message {
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
        }
    }
}

fn vertical_header_group<'a>() -> Column<'a, Message> {
    let title = text("Vertical Curves")
        .width(Length::Fill)
        .size(50)
        .style(Color::from([0.5, 0.5, 0.5]))
        .horizontal_alignment(alignment::Horizontal::Center);
    column![title].spacing(40).width(Length::Fill)
}

fn vertical_input_group(vertical_data: &VerticalData) -> Column<Message> {
    let h_s = 5;
    
    let toggle_station = button(text(">"))
        .on_press(Message::InputMethodToggle);
    let station_modify = text_input(format!("(12+34)").as_str(), &vertical_data.input_station)
        .on_input(Message::StationModify);
    let elevation_modify = text_input(format!("(152)").as_str(), &vertical_data.input_elevation)
        .on_input(Message::ElevationModify);
    let incoming_grade_modify = text_input("(0.01 = 1%)", &vertical_data.input_incoming_grade)
        .on_input(Message::IncomingGradeModify);
    let outgoing_grade_modify = text_input("(-0.02 = -2%)", &vertical_data.input_outgoing_grade)
        .on_input(Message::OutgoingGradeModify);
    let length_modify = text_input("(100)", &vertical_data.input_length)
        .on_input(Message::LengthModify);
    let interval_modify = text_input("(00+25)", &vertical_data.input_station_interval)
        .on_input(Message::StationIntervalModify);
    let toggle_design_standard = button(text("A"))
        .on_press(Message::DesignStandardToggle);
    let toggle_sight = button(text(">"))
        .on_press(Message::SightTypeToggle);
    let design_speed = text_input("(65)", &vertical_data.input_design_speed)
        .on_input(Message::DesignSpeed);
    
    let row_1 = row![text(format!("{:?} Station:", &vertical_data.input_method).as_str()), station_modify, toggle_station].spacing(h_s);
    let row_2 = row![text(format!("{:?} Elevation:", &vertical_data.input_method).as_str()), elevation_modify].spacing(h_s);
    let row_3 = row![text("Incoming Grade:"), incoming_grade_modify].spacing(h_s);
    let row_4 = row![text("Outgoing Grade:"), outgoing_grade_modify].spacing(h_s);
    let row_5 = row![text("Length:"), length_modify].spacing(h_s);
    let row_6 = row![text("Interval:"), interval_modify].spacing(h_s);
    let row_7 = row![text("Design Speed:"), design_speed, toggle_design_standard, toggle_sight].spacing(h_s);

    column![row_1, row_2, row_3, row_4, row_5, row_6, row_7]
        .spacing(10)
        .width(Length::FillPortion(2))
        .padding(10)
    // column![station_modify, elevation_modify, incoming_grade_modify, outgoing_grade_modify]
    // .spacing(10)
    // .max_width(200)
    // .align_items(Alignment::Start)
}

