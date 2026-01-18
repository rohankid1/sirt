use serde::Deserialize;
use serde::de::{self, Deserializer, Error, IntoDeserializer, MapAccess, Visitor};

use crate::error::SirtDeserializeError;
use crate::{Block, Value, parse_input};

struct BlockMapAccess<'a> {
    iter: std::collections::hash_map::Iter<'a, String, Value>,
    value: Option<&'a Value>,
}

impl<'de, 'a> MapAccess<'de> for BlockMapAccess<'a> {
    type Error = SirtDeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.iter.next() {
            self.value = Some(value);
            seed.deserialize(key.as_str().into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let value = self
            .value
            .take()
            .ok_or(SirtDeserializeError::custom("MapAccess error"))?;

        seed.deserialize(ValueDeserializer { value })
    }
}

pub struct BlockDeserializer<'a> {
    block: &'a Block,
}

impl<'de, 'a> Deserializer<'de> for BlockDeserializer<'a> {
    type Error = SirtDeserializeError;

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(BlockMapAccess {
            iter: self.block.fields.iter(),
            value: None,
        })
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_struct("", &[], visitor)
    }

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct enum identifier ignored_any
    }
}

struct ValueDeserializer<'a> {
    value: &'a Value,
}

impl<'de, 'a> Deserializer<'de> for ValueDeserializer<'a> {
    type Error = SirtDeserializeError;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unreachable!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Text(s) => visitor.visit_string(s.clone()),
            other => Err(SirtDeserializeError::custom(format!(
                "expected string, found {other:?}"
            ))),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Int(num) => visitor.visit_i64(*num),
            other => Err(SirtDeserializeError::custom(format!(
                "expected i64, found {other:?}"
            ))),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Bool(b) => visitor.visit_bool(*b),
            other => Err(SirtDeserializeError::custom(format!(
                "expected bool, found {other:?}"
            ))),
        }
    }

    serde::forward_to_deserialize_any! {
        i8 i16 i32 i128 u8 u16 u32 u64 u128 f32 f64 char str
        bytes byte_buf option unit unit_struct newtype_struct
        tuple_struct map struct enum identifier ignored_any seq
        tuple
    }
}

pub fn from_str<'de, T>(input: &str) -> Result<T, SirtDeserializeError>
where
    T: Deserialize<'de>,
{
    let blocks = parse_input(input).map_err(|err| {
        SirtDeserializeError::custom(format!("failed to parse sirt format: {err:?}"))
    })?;

    if blocks.len() < 1 {
        return Err(SirtDeserializeError::custom("expected at least one block"));
    }

    let block = &blocks[0];
    let des = BlockDeserializer { block };
    T::deserialize(des)
}

pub fn from_str_named<'de, T>(input: &str, name: &str) -> Result<T, SirtDeserializeError>
where
    T: Deserialize<'de>,
{
    let blocks = parse_input(input).map_err(|err| {
        SirtDeserializeError::custom(format!("failed to parse sirt format: {err:?}"))
    })?;

    let block = blocks.iter().find(|block| block.get_name() == name).ok_or(
        SirtDeserializeError::custom(format!("couldn't find block with name '{name}'")),
    )?;

    let des = BlockDeserializer { block };
    T::deserialize(des)
}
