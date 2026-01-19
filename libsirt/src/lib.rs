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

#[derive(Debug, PartialEq)]
pub enum Value {
    Text(String),
    Int(i64),
    Bool(bool),
    End,
}

pub mod types {
    pub type Int = i64;
    pub type Bool = bool;
    pub type Text = String;
}

#[derive(Debug, PartialEq)]
pub struct Block {
    name: String,
    fields: HashMap<String, Value>,
}

impl Block {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_fields(&self) -> &HashMap<String, Value> {
        &self.fields
    }
}
