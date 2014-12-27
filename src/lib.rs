
extern crate "rustc-serialize" as serialize;
extern crate regex;

use serialize::json;

pub use param::Param;
pub use builder::Builder;
pub use coercers::{
    Coercer,
    StringCoercer,
    I64Coercer,
    U64Coercer,
    F64Coercer,
    BooleanCoercer,
    NullCoercer,
    ListCoercer,
    ObjectCoercer,
};

pub use mutable_json::MutableJson;

pub type ValicoResult<T> = Result<T, json::Object>;

mod builder;
mod coercers;
mod helpers;
mod param;
mod validation;
mod mutable_json;
