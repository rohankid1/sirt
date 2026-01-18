mod de;
pub mod error;
mod parser;

use pest_derive::Parser;
use std::collections::HashMap;

pub use de::{from_str, from_str_named};
pub use parser::parse_input;

#[derive(Parser)]
#[grammar = "../grammar/grammar.pest"]
pub(crate) struct SirtParser;

#[derive(Debug)]
pub enum Value {
    Text(String),
    Int(i64),
    Bool(bool),
    End,
}

#[derive(Debug)]
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
