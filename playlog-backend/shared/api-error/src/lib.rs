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

impl ApiError {
    pub fn new(code: StatusCode, error_message: impl Into<String>) -> Self {
        Self {
            code,
            errors: vec![error_message.into()],
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

fn collect_validation_errors(
    parent: Option<&str>,
    errors: &ValidationErrors,
    messages: &mut Vec<String>,
) {
    for (field, kind) in errors.errors() {
        let full_field = match parent {
            Some(parent) => format!("{parent}.{field}"),
            None => field.to_string(),
        };

        match kind {
            ValidationErrorsKind::Field(field_errors) => {
                for error in field_errors {
                    if let Some(message) = &error.message {
                        messages.push(format!("{full_field}: {message}"));
                    } else {
                        messages.push(format!("{full_field}: validation failed ({})", error.code));
                    }
                }
            }

            ValidationErrorsKind::Struct(struct_errors) => {
                collect_validation_errors(Some(&full_field), struct_errors, messages);
            }

            ValidationErrorsKind::List(list_errors) => {
                for (index, list_item_errors) in list_errors {
                    let indexed = format!("{full_field}[{index}]");
                    collect_validation_errors(Some(&indexed), list_item_errors, messages);
                }
            }
        }
    }
}
