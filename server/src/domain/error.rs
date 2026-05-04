use std::collections::HashMap;

pub type FieldErrors = HashMap<String, Vec<String>>;

pub enum ServerError {
    Internal,
    DatabaseError,
    ValidationError(FieldErrors),
}
