use anyhow::Result;
use iced::{
    alignment::{self},
    event,
    font::{self, Font},
    keyboard::{self, KeyCode, Modifiers},
    subscription,
    theme::Theme,
    widget::{
        button, column, container, row, scrollable, text, text_input, Column, Row, Rule, Text,
    },
    window::{self, Mode},
    Application, Color, Command, Element, Event, Length, Subscription,
};
use once_cell::sync::Lazy;

pub mod horizontal;
pub mod vertical;

use crate::datatypes::*;
use crate::export::*;
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
    FileDialog,
    Directory(String),
    ExportText,
    ExportPDF,
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
                vertical_data.success_flags = [ExportSuccess::None; 2];

                match message {
                    Message::FileDialog => {
                        vertical_data.input_directory = save_to();
                    }
                    Message::Directory(raw_data) => {
                        vertical_data.input_directory = raw_data;
                    }
                    Message::InputMethodToggle => {
                        vertical_data.input_method = vertical_data.input_method.next();
                    }
                    Message::StationModify(raw_input) => {
                        vertical_data.input_station = raw_input;
                    }
                    Message::ElevationModify(raw_input) => {
                        vertical_data.input_elevation = raw_input;
                    }
                    Message::IncomingGradeModify(raw_input) => {
                        vertical_data.input_incoming_grade = raw_input;
                    }
                    Message::OutgoingGradeModify(raw_input) => {
                        vertical_data.input_outgoing_grade = raw_input;
                    }
                    Message::LengthModify(raw_input) => {
                        vertical_data.input_length = raw_input;
                    }
                    Message::StationIntervalModify(raw_input) => {
                        vertical_data.input_station_interval = raw_input;
                    }
                    Message::DesignStandardToggle => {
                        vertical_data.input_design_standard =
                            vertical_data.input_design_standard.next();
                    }
                    Message::SightTypeToggle => {
                        vertical_data.input_sight_type = vertical_data.input_sight_type.next();
                    }
                    Message::DesignSpeed(raw_input) => {
                        vertical_data.input_design_speed = raw_input;
                    }
                    Message::SustainedDowngradeCheck(raw_input) => {
                        vertical_data.sustained_downgrade = raw_input;
                    }
                    Message::ObstacleStation(raw_input) => {
                        vertical_data.input_obstacle_station = raw_input;
                    }
                    Message::ObstacleElevation(raw_input) => {
                        vertical_data.input_obstacle_elevation = raw_input;
                    }
                    Message::ObstacleTypeToggle => {
                        vertical_data.input_obstacle_type =
                            vertical_data.input_obstacle_type.next();
                    }
                    Message::AddObstacle => {
                        let _ = self.add_to_list();
                    }
                    Message::RemoveObstacle => {
                        vertical_data.obstacles.interval.pop();
                    }
                    Message::ExportText => match vertical_data.export_txt() {
                        Ok(w) => vertical_data.success_flags[0] = ExportSuccess::Success,
                        Err(e) => vertical_data.success_flags[0] = ExportSuccess::Failure,
                    },
                    Message::ExportPDF => match vertical_data.export_pdf() {
                        Ok(w) => vertical_data.success_flags[1] = ExportSuccess::Success,
                        Err(e) => vertical_data.success_flags[1] = ExportSuccess::Failure,
                    },
                    _ => (),
                };
                Command::batch(vec![generic])
            }
            CurveSolver::Horizontal(horizontal_data) => {
                horizontal_data.success_flags = [ExportSuccess::None; 2];

                match message {
                    Message::FileDialog => {
                        horizontal_data.input_directory = save_to();
                    }
                    Message::Directory(raw_data) => {
                        horizontal_data.input_directory = raw_data;
                    }
                    Message::BuildMethodToggle => {
                        horizontal_data.input_build_method =
                            horizontal_data.input_build_method.next();
                    }
                    Message::DesignStandardToggle => {
                        horizontal_data.input_design_standard =
                            horizontal_data.input_design_standard.next();
                    }
                    Message::SightTypeToggle => {
                        horizontal_data.input_sight_type = horizontal_data.input_sight_type.next();
                    }
                    Message::StationMethodToggle => {
                        horizontal_data.input_station_method =
                            horizontal_data.input_station_method.next();
                    }
                    Message::CurveAngleModify(raw_data) => {
                        horizontal_data.input_curve_angle = raw_data;
                    }
                    Message::DesignSpeed(raw_data) => {
                        horizontal_data.input_design_speed = raw_data;
                    }
                    Message::LengthModify(raw_data) => {
                        horizontal_data.input_length = raw_data;
                    }
                    Message::RadiusModify(raw_data) => {
                        horizontal_data.input_radius = raw_data;
                    }
                    Message::TangentModify(raw_data) => {
                        horizontal_data.input_tangent = raw_data;
                    }
                    Message::StationIntervalModify(raw_data) => {
                        horizontal_data.input_station_interval = raw_data;
                    }
                    Message::StationModify(raw_data) => {
                        horizontal_data.input_station = raw_data;
                    }
                    Message::MModify(raw_data) => {
                        horizontal_data.input_m = raw_data;
                    }
                    Message::SustainedDowngradeCheck(raw_input) => {
                        horizontal_data.sustained_downgrade = raw_input;
                    }
                    Message::ExportText => match horizontal_data.export_txt() {
                        Ok(w) => horizontal_data.success_flags[0] = ExportSuccess::Success,
                        Err(e) => horizontal_data.success_flags[0] = ExportSuccess::Failure,
                    },
                    Message::ExportPDF => match horizontal_data.export_pdf() {
                        Ok(w) => horizontal_data.success_flags[1] = ExportSuccess::Success,
                        Err(e) => horizontal_data.success_flags[1] = ExportSuccess::Failure,
                    },
                    // Message::PinStation(raw_data) => {
                    //     horizontal_data.input_pin_station = raw_data;
                    //
                    // }
                    // Message::AddPin => {
                    //     let _ = self.add_to_list();
                    //
                    // }
                    // Message::RemovePin => {
                    //     horizontal_data.pin.interval.pop();
                    //
                    // }
                    _ => (),
                };
                Command::batch(vec![generic])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let body = match self {
            CurveSolver::Vertical(vertical_data) => {
                row![
                    vertical_data.vertical_input_group(),
                    Rule::vertical(40),
                    vertical_data.vertical_output_group()
                ]
            }
            CurveSolver::Horizontal(horizontal_data) => {
                row![
                    horizontal_data.horizontal_input_group(),
                    Rule::vertical(40),
                    horizontal_data.horizontal_output_group()
                ]
            }
        };

        scrollable(
            container(column![self.task_row(), body].spacing(H_S))
                .width(Length::Fill)
                .padding(40)
                .center_x(),
        )
        .into()
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
    fn task_row(&self) -> Column<'_, Message> {
        let title = match self {
            CurveSolver::Horizontal(_) => "Horizontal Curves (Pre-Release)",
            CurveSolver::Vertical(_) => "Vertical Curves (Pre-Release)",
        };
        let file = match self {
            CurveSolver::Horizontal(data) => &data.input_directory,
            CurveSolver::Vertical(data) => &data.input_directory,
        };

        let title_header = text(title)
            .width(Length::Fill)
            .size(50)
            .style(Color::from([0.5, 0.5, 0.5]))
            .horizontal_alignment(alignment::Horizontal::Center);
        let task_row = row![
            button("Switch Curve Type").on_press(Message::SwitchCurveType),
            // Space::with_width(Length::Fill),
            button(exit_icon()).on_press(Message::FileDialog),
            text_input("File Directory", file)
                .on_input(Message::Directory)
                .width(Length::Fill),
            self.display_export(),
            // button(self.display_export_txt())
            //     .on_press(Message::ExportText)
            //     .width(42),
            // button(text(".pdf").horizontal_alignment(alignment::Horizontal::Center))
            //     .on_press(Message::ExportPDF)
            //     .width(42),
            // button(text(".xlsx").horizontal_alignment(alignment::Horizontal::Center))
            //     .on_press(Message::ExportXLSX)
            //     .width(42)
        ]
        .width(Length::Fill)
        .spacing(H_S);
        column![title_header, task_row]
            .spacing(H_S)
            .width(Length::Fill)
    }

    fn display_export(&self) -> Row<'_, Message> {
        let mut export_row = row![].spacing(H_S);
        let labels = [".txt", ".pdf"];
        let msg = [Message::ExportText, Message::ExportPDF];
        let binding = match self {
            Self::Horizontal(data) => data.success_flags,
            Self::Vertical(data) => data.success_flags,
        };

        for (i, k) in binding.iter().enumerate() {
            match k {
                ExportSuccess::Failure => {
                    export_row =
                        export_row.push(button(exclam_icon()).on_press(msg[i].clone()).width(42))
                }
                ExportSuccess::None => {
                    export_row =
                        export_row.push(button(labels[i]).on_press(msg[i].clone()).width(42))
                }
                ExportSuccess::Success => {
                    export_row = export_row
                        .push(button(good_check_icon()).on_press(msg[i].clone()).width(42))
                }
            };
        }
        export_row
    }

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

fn exit_icon() -> Text<'static> {
    icon2('\u{2e}')
}
const SUBTITLE_SIZE: u16 = 22;

fn subtitle(str: &str) -> Text<'static> {
    text(str).size(SUBTITLE_SIZE)
}
