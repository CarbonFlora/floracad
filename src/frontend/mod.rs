use anyhow::Result;
use iced::alignment::{self};
use iced::font::{self, Font};
use iced::keyboard::{self, KeyCode, Modifiers};
use iced::theme::Theme;
use iced::widget::{button, column, container, row, scrollable, text_input};
use iced::widget::{text, Text};
use iced::window::{self, Mode};
use iced::{event, Event};
use iced::{subscription, Subscription};
use iced::{Application, Element};
use iced::{Color, Command, Length};
use once_cell::sync::Lazy;

pub mod horizontal;
pub mod vertical;

use crate::datatypes::*;
use crate::frontend::horizontal::*;
use crate::frontend::vertical::*;
use crate::horizontal::*;
use crate::vertical::*;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

const H_S: u16 = 7;

#[derive(Debug)]
pub enum CurveSolver {
    Vertical(VerticalData),
    Horizontal(HorizontalData),
}

#[derive(Debug, Clone)]
pub enum Message {
    // generic
    FontLoaded(Result<(), font::Error>),
    FullScreenToggle(Mode),
    SwitchCurveType,
    SustainedDowngradeCheck(bool),
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
    ObstacleStation(String),
    ObstacleElevation(String),
    ObstacleTypeToggle,
    AddObstacle,
    RemoveObstacle,
    // Horizontal
    StationMethodToggle,
    BuildMethodToggle,
    RadiusModify(String),
    CurveAngleModify(String),
    TangentModify(String),
    MModify(String),
    // PinStation(String),
    // AddPin,
    // RemovePin,
}

impl Application for CurveSolver {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (CurveSolver, Command<Message>) {
        (
            CurveSolver::Vertical(VerticalData::default()),
            Command::batch([
                font::load(include_bytes!("../../fonts/Arrows.ttf").as_slice())
                    .map(Message::FontLoaded),
                font::load(include_bytes!("../../fonts/Byom-Icons-Trial.ttf").as_slice())
                    .map(Message::FontLoaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Zi's Curve Solver")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let generic = match message {
            Message::FullScreenToggle(mode) => window::change_mode(mode),
            Message::SwitchCurveType => {
                self.next_page();
                Command::none()
            }
            _ => Command::none(),
        };

        match self {
            CurveSolver::Vertical(vertical_data) => {
                let specific = match message {
                    Message::InputMethodToggle => {
                        vertical_data.input_method = vertical_data.input_method.next();
                        Command::none()
                    }
                    Message::StationModify(raw_input) => {
                        vertical_data.input_station = raw_input;
                        Command::none()
                    }
                    Message::ElevationModify(raw_input) => {
                        vertical_data.input_elevation = raw_input;
                        Command::none()
                    }
                    Message::IncomingGradeModify(raw_input) => {
                        vertical_data.input_incoming_grade = raw_input;
                        Command::none()
                    }
                    Message::OutgoingGradeModify(raw_input) => {
                        vertical_data.input_outgoing_grade = raw_input;
                        Command::none()
                    }
                    Message::LengthModify(raw_input) => {
                        vertical_data.input_length = raw_input;
                        Command::none()
                    }
                    Message::StationIntervalModify(raw_input) => {
                        vertical_data.input_station_interval = raw_input;
                        Command::none()
                    }
                    Message::DesignStandardToggle => {
                        vertical_data.input_design_standard =
                            vertical_data.input_design_standard.next();
                        Command::none()
                    }
                    Message::SightTypeToggle => {
                        vertical_data.input_sight_type = vertical_data.input_sight_type.next();
                        Command::none()
                    }
                    Message::DesignSpeed(raw_input) => {
                        vertical_data.input_design_speed = raw_input;
                        Command::none()
                    }
                    Message::SustainedDowngradeCheck(raw_input) => {
                        vertical_data.sustained_downgrade = raw_input;
                        Command::none()
                    }
                    Message::ObstacleStation(raw_input) => {
                        vertical_data.input_obstacle_station = raw_input;
                        Command::none()
                    }
                    Message::ObstacleElevation(raw_input) => {
                        vertical_data.input_obstacle_elevation = raw_input;
                        Command::none()
                    }
                    Message::ObstacleTypeToggle => {
                        vertical_data.input_obstacle_type =
                            vertical_data.input_obstacle_type.next();
                        Command::none()
                    }
                    Message::AddObstacle => {
                        let _ = self.add_to_list();
                        Command::none()
                    }
                    Message::RemoveObstacle => {
                        vertical_data.obstacles.interval.pop();
                        Command::none()
                    }
                    _ => Command::none(),
                };
                Command::batch(vec![generic, specific])
            }
            CurveSolver::Horizontal(horizontal_data) => {
                let specific = match message {
                    Message::BuildMethodToggle => {
                        horizontal_data.input_build_method =
                            horizontal_data.input_build_method.next();
                        Command::none()
                    }
                    Message::DesignStandardToggle => {
                        horizontal_data.input_design_standard =
                            horizontal_data.input_design_standard.next();
                        Command::none()
                    }
                    Message::SightTypeToggle => {
                        horizontal_data.input_sight_type = horizontal_data.input_sight_type.next();
                        Command::none()
                    }
                    Message::StationMethodToggle => {
                        horizontal_data.input_station_method =
                            horizontal_data.input_station_method.next();
                        Command::none()
                    }
                    Message::CurveAngleModify(raw_data) => {
                        horizontal_data.input_curve_angle = raw_data;
                        Command::none()
                    }
                    Message::DesignSpeed(raw_data) => {
                        horizontal_data.input_design_speed = raw_data;
                        Command::none()
                    }
                    Message::LengthModify(raw_data) => {
                        horizontal_data.input_length = raw_data;
                        Command::none()
                    }
                    Message::RadiusModify(raw_data) => {
                        horizontal_data.input_radius = raw_data;
                        Command::none()
                    }
                    Message::TangentModify(raw_data) => {
                        horizontal_data.input_tangent = raw_data;
                        Command::none()
                    }
                    Message::StationIntervalModify(raw_data) => {
                        horizontal_data.input_station_interval = raw_data;
                        Command::none()
                    }
                    Message::StationModify(raw_data) => {
                        horizontal_data.input_station = raw_data;
                        Command::none()
                    }
                    Message::MModify(raw_data) => {
                        horizontal_data.input_m = raw_data;
                        Command::none()
                    }
                    Message::SustainedDowngradeCheck(raw_input) => {
                        horizontal_data.sustained_downgrade = raw_input;
                        Command::none()
                    }
                    // Message::PinStation(raw_data) => {
                    //     horizontal_data.input_pin_station = raw_data;
                    //     Command::none()
                    // }
                    // Message::AddPin => {
                    //     let _ = self.add_to_list();
                    //     Command::none()
                    // }
                    // Message::RemovePin => {
                    //     horizontal_data.pin.interval.pop();
                    //     Command::none()
                    // }
                    _ => Command::none(),
                };
                Command::batch(vec![generic, specific])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            CurveSolver::Vertical(vertical_data) => {
                let title = vertical_header_group();
                let body = row![
                    vertical_data.vertical_input_group(),
                    vertical_data.vertical_output_group()
                ];

                scrollable(
                    container(column![title, body].spacing(10))
                        .width(Length::Fill)
                        .padding(40)
                        .center_x(),
                )
                .into()
            }
            CurveSolver::Horizontal(horizontal_data) => {
                let title = horizontal_header_group();
                let body = row![
                    horizontal_data.horizontal_input_group(),
                    horizontal_data.horizontal_output_group()
                ];

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

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| match (event, status) {
            // (
            //     Event::Keyboard(keyboard::Event::KeyPressed {
            //         key_code: keyboard::KeyCode::Tab,
            //         modifiers,
            //         ..
            //     }),
            //     event::Status::Ignored,
            // ) => Some(Message::TabPressed {
            //     shift: modifiers.shift(),
            // }),
            (
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code,
                    modifiers: Modifiers::SHIFT,
                }),
                event::Status::Ignored,
            ) => match key_code {
                KeyCode::Up => Some(Message::FullScreenToggle(Mode::Fullscreen)),
                KeyCode::Down => Some(Message::FullScreenToggle(Mode::Windowed)),
                _ => None,
            },
            _ => None,
        })
    }
}

impl CurveSolver {
    pub fn next_page(&mut self) {
        match self {
            CurveSolver::Vertical(vertical_data) => {
                *self = CurveSolver::Horizontal(HorizontalData::default())
            }
            CurveSolver::Horizontal(horizontal_data) => {
                *self = CurveSolver::Vertical(VerticalData::default())
            }
        }
    }

    pub fn add_to_list(&mut self) -> Result<()> {
        //this is a hack
        match self {
            CurveSolver::Vertical(vertical_data) => {
                let value = coerce_station_value(&vertical_data.input_obstacle_station)?;
                let elevation = coerce_elevation(&vertical_data.input_obstacle_elevation)?;
                let station = Station { value, elevation };
                vertical_data
                    .obstacles
                    .interval
                    .push((station, vertical_data.input_obstacle_type));
            }
            CurveSolver::Horizontal(horizontal_data) => {
                let value = coerce_station_value(&horizontal_data.input_pin_station)?;

                horizontal_data.pin.interval.push(Station {
                    value,
                    elevation: 0.0,
                });
            }
        }
        Ok(())
    }
}

impl From<VerticalDefinition> for HorizontalStationDefinition {
    fn from(value: VerticalDefinition) -> Self {
        match value {
            VerticalDefinition::PVC => HorizontalStationDefinition::PC,
            VerticalDefinition::PVI => HorizontalStationDefinition::PI,
            VerticalDefinition::PVT => HorizontalStationDefinition::PT,
        }
    }
}

impl From<HorizontalStationDefinition> for VerticalDefinition {
    fn from(value: HorizontalStationDefinition) -> Self {
        match value {
            HorizontalStationDefinition::PC => VerticalDefinition::PVC,
            HorizontalStationDefinition::PI => VerticalDefinition::PVI,
            HorizontalStationDefinition::PT => VerticalDefinition::PVT,
        }
    }
}

// Generic Text
fn stext(character: char) -> Text<'static> {
    text(character.to_string())
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(20)
}

// Fonts
const ICONS: Font = Font::with_name("Arrows");
const ICONS2: Font = Font::with_name("Byom Icons");

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(20)
}

fn icon2(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS2)
        .horizontal_alignment(alignment::Horizontal::Center)
        .width(20)
}

fn cycle_icon() -> Text<'static> {
    icon('\u{79}') //5a
}

fn up_arrow_icon() -> Text<'static> {
    icon('\u{63}')
}

fn down_arrow_icon() -> Text<'static> {
    icon('\u{64}')
}

fn exclam_icon() -> Text<'static> {
    icon2('\u{21}')
}

fn good_check_icon() -> Text<'static> {
    icon2('\u{56}')
}

fn right_carrot_icon() -> Text<'static> {
    icon2('\u{51}')
}

fn notification_icon() -> Text<'static> {
    icon2('\u{49}')
}

const SUBTITLE_SIZE: u16 = 22;

fn subtitle(str: &str) -> Text<'static> {
    text(str).size(SUBTITLE_SIZE)
}
