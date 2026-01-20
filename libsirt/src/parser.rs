use crate::error::ParseError;
use crate::types::List;
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
        Rule::list => Ok(Value::List(parse_list(pair)?)),
        _ => unreachable!(),
    }
}

fn parse_list(pair: Pair<'_, Rule>) -> Result<List<Value>, ParseError<'_>> {
    let mut list = List::with_capacity(1);

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::value_list {
            for v in inner.into_inner() {
                list.push(parse_value(v)?);
            }
        }
    }

    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Value::*;

    #[test]
    fn test_input_with_empty_blocks() {
        let input = r#"
            Person {}
        "#;
        let input2 = r#"
            One {}
            Two {}
            Three {}
            "#;

        let parser = parse_input(input);
        let parser2 = parse_input(input2);

        assert!(parser.is_ok());
        assert!(parser2.is_ok());

        let mut block1 = parser.unwrap().into_iter();
        let mut block2 = parser2.unwrap().into_iter();

        assert_eq!(
            block1.next(),
            Some(Block {
                name: "Person".to_owned(),
                fields: HashMap::default()
            })
        );

        assert_eq!(block1.next(), None);

        assert_eq!(
            block2.next(),
            Some(Block {
                name: "One".to_owned(),
                fields: HashMap::default()
            })
        );

        assert_eq!(
            block2.next(),
            Some(Block {
                name: "Two".to_owned(),
                fields: HashMap::default()
            })
        );

        assert_eq!(
            block2.next(),
            Some(Block {
                name: "Three".to_owned(),
                fields: HashMap::default()
            })
        );

        assert_eq!(block2.next(), None);
    }

    #[test]
    fn test_input_with_fields() {
        let input = "App { isRunning: bool(false) num: int(64) }";
        let parser = parse_input(input);

        assert!(parser.is_ok());

        let mut iter = parser.unwrap().into_iter();

        assert_eq!(
            iter.next(),
            Some(Block {
                name: "App".to_string(),
                fields: HashMap::from([
                    ("isRunning".to_owned(), Value::Bool(false)),
                    ("num".to_string(), Value::Int(64))
                ])
            })
        );

        assert!(iter.next().is_none());
    }

    #[test]
    fn test_list() {
        let input = r#"NumList { arr: list(int(1), int(2), int(3)) }"#;
        let parser = parse_input(input);

        assert!(parser.is_ok());

        let mut iter = parser.unwrap().into_iter();

        assert_eq!(
            iter.next(),
            Some(Block {
                name: "NumList".to_string(),
                fields: HashMap::from([(
                    "arr".to_string(),
                    Value::List(vec![Int(1), Int(2), Int(3)])
                )])
            })
        );
    }
}
