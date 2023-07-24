use iced::{Settings, Application};

use floracad::frontend::CurveSolver;

fn main() -> Result<(), iced::Error> {
    Ok(CurveSolver::run(Settings::default())?)
}
