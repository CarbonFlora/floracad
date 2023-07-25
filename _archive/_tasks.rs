// use iced::alignment::{self, Alignment};
// use iced::event::{self, Event};
// use iced::keyboard::{self, KeyCode, Modifiers};
// use iced::subscription;
// use iced::theme::{self, Theme};
// use iced::widget::{self, button, checkbox, column, container, row, scrollable, text, text_input, Text};
// use iced::window;
// use iced::{Application, Element};
// use iced::{Color, Command, Length, Settings, Subscription};

// // #[derive(Debug, Clone, Serialize, Deserialize)]
// #[derive(Debug, Clone)]
// pub struct Task {
//     description: String,
//     completed: bool,

//     // #[serde(skip)]
//     state: TaskState,
// }

// #[derive(Debug, Clone)]
// pub enum TaskState {
//     Idle,
//     Editing,
// }

// impl Default for TaskState {
//     fn default() -> Self {
//         Self::Idle
//     }
// }


// #[derive(Debug, Clone)]
// pub enum TaskMessage {
//     Completed(bool),
//     Edit,
//     DescriptionEdited(String),
//     FinishEdition,
//     Delete,
// }

// impl Task {
//     fn text_input_id(i: usize) -> text_input::Id {
//         text_input::Id::new(format!("task-{i}"))
//     }

//     pub fn new(description: String) -> Self {
//         Task {
//             description,
//             completed: false,
//             state: TaskState::Idle,
//         }
//     }

//     fn update(&mut self, message: TaskMessage) {
//         match message {
//             TaskMessage::Completed(completed) => {
//                 self.completed = completed;
//             }
//             TaskMessage::Edit => {
//                 self.state = TaskState::Editing;
//             }
//             TaskMessage::DescriptionEdited(new_description) => {
//                 self.description = new_description;
//             }
//             TaskMessage::FinishEdition => {
//                 if !self.description.is_empty() {
//                     self.state = TaskState::Idle;
//                 }
//             }
//             TaskMessage::Delete => {}
//         }
//     }

//     pub fn view(&self, i: usize) -> Element<TaskMessage> {
//         match &self.state {
//             TaskState::Idle => {
//                 let checkbox = checkbox(
//                     &self.description,
//                     self.completed,
//                     TaskMessage::Completed,
//                 )
//                 .width(Length::Fill);
//                 // .text_shaping(text::Shaping::Advanced);

//                 row![
//                     checkbox,
//                     button("O")
//                         .on_press(TaskMessage::Edit)
//                         .padding(10)
//                         .style(theme::Button::Text),
//                 ]
//                 .spacing(20)
//                 .align_items(Alignment::Center)
//                 .into()
//             }
//             TaskState::Editing => {
//                 let text_input =
//                     text_input("Describe your task...", &self.description)
//                         .id(Self::text_input_id(i))
//                         .on_input(TaskMessage::DescriptionEdited)
//                         .on_submit(TaskMessage::FinishEdition)
//                         .padding(10);

//                 row![
//                     text_input,
//                     button(
//                         row!["X", "Delete"]
//                             .spacing(10)
//                             .align_items(Alignment::Center)
//                     )
//                     .on_press(TaskMessage::Delete)
//                     .padding(10)
//                     .style(theme::Button::Destructive)
//                 ]
//                 .spacing(20)
//                 .align_items(Alignment::Center)
//                 .into()
//             }
//         }
//     }
// }