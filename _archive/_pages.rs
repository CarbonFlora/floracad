// use iced::alignment::{self, Alignment};
// use iced::event::{self, Event};
// use iced::keyboard::{self, KeyCode, Modifiers};
// use iced::subscription;
// use iced::theme::{self, Theme};
// use iced::widget::{self, button, checkbox, column, container, row, scrollable, text, text_input, Text};
// use iced::window;
// use iced::{Application, Element};
// use iced::{Color, Command, Length, Settings, Subscription};
// use once_cell::sync::Lazy;

// use crate::tasks::*;

// static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

// #[derive(Debug)]
// enum Todos {
//     Loaded(State),
// }

// #[derive(Debug, Default)]
// struct State {
//     input_value: String,
//     // filter: Filter,
//     tasks: Vec<Task>,
//     // dirty: bool,
//     // saving: bool,
// }

// #[derive(Debug, Clone)]
// enum Message {
//     InputChanged(String),
//     PerformCurveCalcs,
//     LeaveInputBox,
//     // TabPressed { shift: bool },
//     // ToggleFullscreen(window::Mode),
// }

// impl Application for Todos {
//     type Message = Message;
//     type Theme = Theme;
//     type Executor = iced::executor::Default;
//     type Flags = ();

//     fn new(_flags: ()) -> (Todos, Command<Message>) {
//         (
//             Todos::Loaded(State { input_value: String::new(), tasks: Vec::new() }),
//             Command::none(),
//         )
//     }

//     fn title(&self) -> String {
//         String::from("Zi's Curve Solver")
//     }

//     fn update(&mut self, message: Message) -> Command<Message> {
//         match self {
//             Todos::Loaded(state) => {
//                 let command = match message {
//                     Message::InputChanged(value) => {
//                         state.input_value = value;

//                         Command::none()
//                     }
//                     Message::LeaveInputBox => {
//                         widget::focus_next()
//                     }
//                     Message::PerformCurveCalcs => {
//                         if !state.input_value.is_empty() {
//                             state
//                                 .tasks
//                                 .push(Task::new(state.input_value.clone()));
//                             state.input_value.clear();
//                         }

//                         Command::none()
//                     }  
//                 };
//                 Command::batch(vec![command])
//             }
//         }
//     }

//     fn view(&self) -> Element<Message> {
//         match self {
//             Todos::Loaded(State {
//                 input_value,
//                 tasks,
//             }) => {
//                 let title = text("Vertical Curves")
//                     .width(Length::Fill)
//                     .size(50)
//                     .style(Color::from([0.5, 0.5, 0.5]))
//                     .horizontal_alignment(alignment::Horizontal::Center);

//                 let input = text_input("Station", input_value)
//                     .id(INPUT_ID.clone())
//                     .on_input(Message::InputChanged)
//                     .on_submit(Message::LeaveInputBox)
//                     .padding(15)
//                     .size(30);

//                 let tasks: Element<_> =
//                     column(
//                         tasks
//                             .iter()
//                             .enumerate()
//                             .map(|(i, task)| {
//                                 task.view(i).map(move |message| {
//                                     Message::TaskMessage(i, message)
//                                 })
//                             })
//                             .collect(),
//                     )
//                     .spacing(10)
//                     .into();

//                 let content = column![title, input, tasks]
//                     .spacing(20)
//                     .max_width(800);

//                 scrollable(
//                     container(content)
//                         .width(Length::Fill)
//                         .padding(40)
//                         .center_x(),
//                 )
//                 .into()
//             }
//         }
//     }

//     // fn new() -> Self {
//     //     Book {}
//     // }

//     // fn title(&self) -> String {
//     //     String::from("Zi's Curve Solver")
//     // }

//     // fn update(&mut self, message: Self::Message) {
//     //     match message {

//     //     }
//     // }

//     // fn view(&self) -> iced::Element<'_, Self::Message> {
//     // //     match self.current_view {
//     // //         Views::Counter => self.counter_layout::<Self::Message>().into(),
//     // //         Views::MainPage => self.main_page_layout::<Self::Message>().into(),
//     // //     }
//     //     self.main_page_layout::<Self::Message>().into()
//     // }
// }