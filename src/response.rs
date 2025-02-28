use serde::Serialize;

#[derive(Serialize)]
pub struct CustomResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> CustomResponse<T> {
    pub fn new(code: u16, message: impl Into<String>, data: Option<T>) -> Self {
        Self {
            code,
            message: message.into(),
            data,
        }
    }
}