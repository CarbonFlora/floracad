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
