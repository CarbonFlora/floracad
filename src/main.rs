use iced::{Application, Settings};

use floracad::frontend::CurveSolver;

fn main() -> Result<(), iced::Error> {
    CurveSolver::run(Settings::default())
}
