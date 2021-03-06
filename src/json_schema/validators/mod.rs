use rustc_serialize::json;
use std::fmt;
use url;

use super::scope;

macro_rules! strict_process {
    ($val:expr, $path:ident, $err:expr) => {{
        let maybe_val = $val;
        if maybe_val.is_none() {
            return val_error!(
                $crate::json_schema::errors::WrongType {
                    path: $path.to_string(),
                    detail: $err.to_string()
                }
            )
        }

        maybe_val.unwrap()
    }}
}

macro_rules! nonstrict_process {
    ($val:expr, $path:ident) => {{
        let maybe_val = $val;
        if maybe_val.is_none() {
            return $crate::json_schema::validators::ValidationState::new()
        }

        maybe_val.unwrap()
    }}
}

macro_rules! val_error{
    ($err:expr) => (
        $crate::json_schema::validators::ValidationState{
            errors: vec![
                Box::new($err)
            ],
            missing: vec![]
        }
    )
}

pub use self::multiple_of::{MultipleOf};
pub use self::maxmin::{Maximum, Minimum};
pub use self::maxmin_length::{MaxLength, MinLength};
pub use self::pattern::{Pattern};
pub use self::maxmin_items::{MaxItems, MinItems};
pub use self::unique_items::{UniqueItems};
pub use self::items::{Items};
pub use self::maxmin_properties::{MaxProperties, MinProperties};
pub use self::required::{Required};
pub use self::properties::{Properties};
pub use self::dependencies::{Dependencies};
pub use self::enum_::{Enum};
pub use self::type_::{Type};
pub use self::of::{AllOf, AnyOf, OneOf};
pub use self::ref_::{Ref};
pub use self::not::{Not};

mod multiple_of;
mod maxmin;
mod maxmin_length;
mod pattern;
mod maxmin_items;
mod unique_items;
pub mod items;
mod maxmin_properties;
mod required;
pub mod properties;
pub mod dependencies;
mod enum_;
pub mod type_;
mod of;
mod ref_;
mod not;
pub mod formats;

#[derive(Debug)]
pub struct ValidationState {
    pub errors: super::super::common::error::ValicoErrors,
    pub missing: Vec<url::Url>
}

impl ValidationState {
    pub fn new() -> ValidationState {
        ValidationState {
            errors: vec![],
            missing: vec![]
        }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.len() == 0
    }

    pub fn is_strictly_valid(&self) -> bool {
        self.errors.len() == 0 && self.missing.len() == 0
    }

    pub fn append(&mut self, second: &mut ValidationState) {
        self.errors.append(&mut second.errors);
        self.missing.append(&mut second.missing);
    }
}

impl json::ToJson for ValidationState {
    fn to_json(&self) -> json::Json {
        let mut map = ::std::collections::BTreeMap::new();
        map.insert("errors".to_string(), json::Json::Array(
            self.errors.iter().map(|err| err.to_json()).collect::<Vec<json::Json>>()
        ));
        map.insert("missing".to_string(), json::Json::Array(
            self.missing.iter().map(|url| url.serialize().to_json()).collect::<Vec<json::Json>>()
        ));
        json::Json::Object(map)
    }
}

pub trait Validator {
    fn validate(&self, item: &json::Json, &str, &scope::Scope) -> ValidationState;
}

impl fmt::Debug for Validator + 'static + Send + Sync {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("<validator>")
    }
}

pub type BoxedValidator = Box<Validator + 'static + Send + Sync>;
pub type Validators = Vec<BoxedValidator>;

impl<T> Validator for T where T: Fn(&json::Json, &str, &scope::Scope) -> ValidationState {
    fn validate(&self, val: &json::Json, path: &str, scope: &scope::Scope) -> ValidationState {
        self(val, path, scope)
    }
}
