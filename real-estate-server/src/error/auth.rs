use super::app_error::AppError;

#[derive(Clone)]
pub enum AuthErrorStatusCode {
    UNAUTHORIZED = 401,
    FORBIDDEN = 403,
}

impl AuthErrorStatusCode {
    fn value(&self) -> i32 {
        match *self {
            AuthErrorStatusCode::UNAUTHORIZED => 401,
            AuthErrorStatusCode::FORBIDDEN => 403,
            _ => 500,
        }
    }

    fn value_str(&self) -> String {
        match *self {
            AuthErrorStatusCode::UNAUTHORIZED => String::from("Unauthorized"),
            AuthErrorStatusCode::FORBIDDEN => String::from("Forbidden"),
            _ => String::from("Internal Server Error"),
        }
    }
}

#[derive(Clone)]
pub struct AuthError {
    pub message: Option<String>,
    pub status_code: AuthErrorStatusCode,
}

impl AppError for AuthError {
    fn message(&self) -> String {
        match self.message.clone() {
            Some(msg) => msg,
            None => String::from(format!(
                "Authentication error. {}",
                self.status_code.value_str()
            )),
        }
    }

    fn status_code(&self) -> i32 {
        self.status_code.value()
    }

    fn in_short(&self) -> String {
        format!(
            "Error Message: {}\n Status Code: {}",
            self.message(),
            self.status_code()
        )
    }
}

unsafe impl Send for AuthError {}
