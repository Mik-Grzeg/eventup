use validator::{ValidationError, ValidationErrors};

pub mod appointments;
pub mod schedules;
pub mod services;

fn add_field_error(
    errors: &mut Result<(), ValidationErrors>,
    field: &'static str,
    error: ValidationError,
) {
    if errors.is_ok() {
        *errors = Err(ValidationErrors::new());
    }
    errors.as_mut().unwrap_err().add(field, error);
}
