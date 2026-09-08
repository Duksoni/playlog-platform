use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Debug)]
pub struct ApiError {
    pub code: StatusCode,
    pub errors: Vec<String>,
}

pub type Result<T> = std::result::Result<T, ApiError>;

impl ApiError {
    pub fn new(code: StatusCode, error_message: impl Into<String>) -> Self {
        Self {
            code,
            errors: vec![error_message.into()],
        }
    }

    pub fn payload_too_large(error_message: impl Into<String>) -> Self {
        Self {
            code: StatusCode::PAYLOAD_TOO_LARGE,
            errors: vec![error_message.into()],
        }
    }

    pub fn internal_error() -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            errors: vec![String::from("Internal server error")],
        }
    }

    pub fn with_errors(code: StatusCode, errors: Vec<String>) -> Self {
        Self { code, errors }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.code, Json(self.errors)).into_response()
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(errors: ValidationErrors) -> Self {
        let mut error_messages = Vec::new();
        collect_validation_errors(None, &errors, &mut error_messages);
        Self::with_errors(StatusCode::BAD_REQUEST, error_messages)
    }
}

pub fn is_row_not_found(error: &sqlx::Error) -> bool {
    matches!(error, sqlx::Error::RowNotFound)
}

pub fn sqlx_error_code(error: &sqlx::Error) -> Option<String> {
    if let sqlx::Error::Database(db_error) = error {
        db_error.code().map(|code| code.to_string())
    } else {
        None
    }
}

pub fn is_unique_violation(error: &sqlx::Error) -> bool {
    sqlx_error_code(error).as_deref() == Some("23505")
}

pub fn is_foreign_key_violation(error: &sqlx::Error) -> bool {
    sqlx_error_code(error).as_deref() == Some("23503")
}

pub fn is_mongo_duplicate_key(error: &mongodb::error::Error) -> bool {
    matches!(
        error.kind.as_ref(),
        mongodb::error::ErrorKind::Write(mongodb::error::WriteFailure::WriteError(write_error))
            if write_error.code == 11000
    )
}

fn collect_validation_errors(
    parent: Option<&str>,
    errors: &ValidationErrors,
    messages: &mut Vec<String>,
) {
    errors.errors().iter().for_each(|(field, kind)| {
        let full_field = match parent {
            Some(parent) => format!("{parent}.{field}"),
            None => field.to_string(),
        };
        match kind {
            ValidationErrorsKind::Field(field_errors) => {
                field_errors.iter().for_each(|error| {
                    messages.push(if let Some(message) = &error.message {
                        format!("{full_field}: {message}")
                    } else {
                        format!("{full_field}: validation failed ({})", error.code)
                    })
                });
            }
            ValidationErrorsKind::Struct(struct_errors) => {
                collect_validation_errors(Some(&full_field), struct_errors, messages);
            }
            ValidationErrorsKind::List(list_errors) => {
                list_errors.iter().for_each(|(index, errors)| {
                    let indexed = format!("{full_field}[{index}]");
                    collect_validation_errors(Some(&indexed), errors, messages);
                })
            }
        }
    })
}
