use serde::de;
use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError<'a> {
    Input,
    Block,
    Value,
    Field,
    Other(&'a str),
}

impl<'a> Error for ParseError<'a> {}

impl<'a> std::fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParseError::Input => "Input",
                ParseError::Block => "Block",
                ParseError::Value => "Value",
                ParseError::Field => "Field",
                ParseError::Other(reason) => reason,
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct SirtDeserializeError(String);

impl de::Error for SirtDeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self(msg.to_string())
    }
}

impl std::fmt::Display for SirtDeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SirtDeserializeError {}
