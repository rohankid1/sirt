use crate::error::ParseError;
use crate::{Block, Rule, SirtParser, Value};

use pest::Parser;
use pest::iterators::Pair;
use std::collections::HashMap;

pub fn parse_input(input: &str) -> Result<Vec<Block>, ParseError<'_>> {
    let mut blocks = Vec::new();

    let mut pairs = SirtParser::parse(Rule::input, input).map_err(|_| ParseError::Input)?;

    if let Some(pairs) = pairs.next() {
        for pair in pairs.into_inner() {
            if pair.as_rule() == Rule::block {
                blocks.push(parse_block(pair)?);
            }
        }
    }

    Ok(blocks)
}

fn parse_block(pair: Pair<'_, Rule>) -> Result<Block, ParseError<'_>> {
    let mut inner = pair.into_inner();
    let mut fields = HashMap::new();

    let name = inner.next().ok_or(ParseError::Block)?.as_str().to_string();

    for field in inner {
        let mut parts = field.into_inner();
        let key = parts.next().ok_or(ParseError::Field)?.as_str().to_string();
        let value = parse_value(parts.next().ok_or(ParseError::Value)?)?;
        fields.insert(key, value);
    }

    Ok(Block { name, fields })
}

fn parse_value(pair: Pair<'_, Rule>) -> Result<Value, ParseError<'_>> {
    match pair.as_rule() {
        Rule::text => {
            let s = pair.into_inner().next().ok_or(ParseError::Value)?.as_str();
            let txt = s[1..s.len() - 1].to_string();
            Ok(Value::Text(txt))
        }
        Rule::int => {
            let s = pair.into_inner().next().ok_or(ParseError::Value)?.as_str();
            let n: i64 = s
                .parse()
                .map_err(|_| ParseError::Other("i64 parse error"))?;
            Ok(Value::Int(n))
        }
        Rule::bool => {
            let s = pair.as_str();
            let s: &String = &s[5..].chars().take_while(|c| *c != ')').collect();

            match s.as_str() {
                "true" => Ok(Value::Bool(true)),
                "false" => Ok(Value::Bool(false)),
                "yes" => Ok(Value::Bool(true)),
                "no" => Ok(Value::Bool(false)),
                _ => Err(ParseError::Other("bool parse error")),
            }
        }
        _ => unreachable!(),
    }
}
