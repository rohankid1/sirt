#![doc = include_str!("../README.md")]

mod de;
pub mod error;
mod parser;
mod se;

use pest_derive::Parser;
use std::collections::HashMap;

pub use de::{from_str, from_str_named, from_str_named_iter};
pub use parser::parse_input;
pub use se::{to_pretty_string, to_string};

#[derive(Parser)]
#[grammar = "../grammar/grammar.pest"]
pub(crate) struct SirtParser;

/// An enum representing supported data types
/// in Sirt.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Text(String),
    Int(i64),
    Bool(bool),
    List(Vec<Value>),
}

/// Module containing the primitive types of Sirt.
/// Note that these are simply aliases to Rust's basic
/// data types, these are for easy mapping from Sirt's
/// types.
pub mod types {
    pub type Int = i64;
    pub type Bool = bool;
    pub type Text = String;
    pub type List<T> = Vec<T>;
    pub type ListText = Vec<String>;
    pub type ListInt = Vec<Int>;
    pub type ListBool = Vec<bool>;
}

/// A type that represents a Block in Sirt.
///
/// A Block in Sirt consists of multiple
/// fields (or none), starting with an
/// identifier, followed by a type with a
/// value.
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    name: String,
    fields: HashMap<String, Value>,
}

impl Block {
    /// Returns the name of the Block
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Returns a reference to the `fields` field.
    pub fn get_fields(&self) -> &HashMap<String, Value> {
        &self.fields
    }
}
