use anyhow::Result;
use genpdf::fonts::{FontData, FontFamily};
use native_dialog::FileDialog;

pub mod horizontal;
pub mod vertical;

#[derive(Debug, Clone, Copy, Default)]
pub enum ExportSuccess {
    Failure,
    #[default]
    None,
    Success,
}

pub fn save_to() -> String {
    match FileDialog::new().show_save_single_file() {
        Err(e) => "".to_string(),
        Ok(w) => w
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string(),
    }
}

pub fn liberation_sans() -> Result<FontFamily<FontData>> {
    Ok(FontFamily {
        regular: FontData::new(
            include_bytes!("../../fonts/LiberationSans-Regular.ttf").to_vec(),
            None,
        )?,
        bold: FontData::new(
            include_bytes!("../../fonts/LiberationSans-Bold.ttf").to_vec(),
            None,
        )?,
        italic: FontData::new(
            include_bytes!("../../fonts/LiberationSans-Italic.ttf").to_vec(),
            None,
        )?,
        bold_italic: FontData::new(
            include_bytes!("../../fonts/LiberationSans-BoldItalic.ttf").to_vec(),
            None,
        )?,
    })
}
