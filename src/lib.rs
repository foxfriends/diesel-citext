//! Diesel support for Postgres citext Extension

#![allow(proc_macro_derive_resolution_fallback)]
extern crate diesel;

#[cfg(feature = "with-actix-web")]
extern crate actix_web;

pub mod sql_types;
pub mod types;
mod expression_methods;

pub mod prelude {
    pub use crate::sql_types::Citext;
    pub use crate::types::CiString;
    pub use crate::expression_methods::CitextExpressionMethods;
}
