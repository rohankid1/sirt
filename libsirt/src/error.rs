use serde::{de, ser};
use std::error::Error;

/// An error representing possible
/// values that the parser could
/// return.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError<'a> {
    Input(String),
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
                ParseError::Input(i) => i,
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

#[derive(Debug, Clone)]
pub struct SirtSerializeError(String);

impl ser::Error for SirtSerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self(msg.to_string())
    }
}

impl std::fmt::Display for SirtSerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SirtSerializeError {}
