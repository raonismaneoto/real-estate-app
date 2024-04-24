use std::sync::Arc;

pub trait AppError {
    fn message(&self) -> String;
    fn status_code(&self) -> i32;
    fn in_short(&self) -> String;
}

pub type DynAppError = Box<dyn AppError + Send + Sync>;
