use serialize::json;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct MaxProperties {
    pub length: u64
}

impl super::Validator for MaxProperties {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, _scope: &scope::Scope) -> super::ValidationState {
        let object = strict_process!(val.as_object(), path, strict, "The value must be an object");

        if (object.len() as u64) <= self.length {
            super::ValidationState::new()
        } else {
            val_error!(
                errors::MaxProperties {
                    path: path.to_string()
                }
            )
        }
    }
}

#[allow(missing_copy_implementations)]
pub struct MinProperties {
    pub length: u64
}

impl super::Validator for MinProperties {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, _scope: &scope::Scope) -> super::ValidationState {
        let object = strict_process!(val.as_object(), path, strict, "The value must be an object");

        if (object.len() as u64) >= self.length {
            super::ValidationState::new()
        } else {
            val_error!(
                errors::MinProperties {
                    path: path.to_string()
                }
            )
        }
    }
}