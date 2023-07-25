use iced::{Settings, Application};

use floracad::frontend::CurveSolver;

fn main() -> Result<(), iced::Error> {
    CurveSolver::run(Settings::default())
}
