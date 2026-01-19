use crate::error::SirtSerializeError;
use serde::{
    Serialize,
    ser::{self, Impossible, SerializeStruct},
};

pub struct SirtSerializer {
    output: String,
    prettify: bool,
}

impl SirtSerializer {
    fn new(name: &str, prettify: bool) -> Self {
        let mut output = String::new();
        output.push_str(&format!("{name} {{ "));

        if prettify {
            output.push_str("\n");
        }

        Self { output, prettify }
    }

    fn complete(mut self) -> String {
        self.output
            .push_str(&format!("}}{}", if self.prettify { "\n" } else { "" }));
        self.output
    }

    fn newline(&mut self) {
        self.output.push('\n');
    }
}

pub struct SirtStructSerializer<'a> {
    ser: &'a mut SirtSerializer,
}

impl<'a> SerializeStruct for SirtStructSerializer<'a> {
    type Ok = ();
    type Error = SirtSerializeError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.ser.output.push_str(&format!(
            "{}{key}: ",
            if self.ser.prettify { "\t" } else { "" }
        ));
        value.serialize(&mut *self.ser)?;
        if self.ser.prettify {
            self.ser.newline();
        }
        self.ser.output.push(' ');
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::Serializer for &'a mut SirtSerializer {
    type Error = SirtSerializeError;
    type Ok = ();
    type SerializeStruct = SirtStructSerializer<'a>;
    type SerializeMap = Impossible<(), SirtSerializeError>;
    type SerializeSeq = Impossible<(), SirtSerializeError>;
    type SerializeStructVariant = Impossible<(), SirtSerializeError>;
    type SerializeTuple = Impossible<(), SirtSerializeError>;
    type SerializeTupleStruct = Impossible<(), SirtSerializeError>;
    type SerializeTupleVariant = Impossible<(), SirtSerializeError>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&format!("bool({v})"));
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&format!("int({v})"));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output.push_str(&format!("text(\"{v}\")"));
        Ok(())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SirtStructSerializer { ser: self })
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn collect_map<K, V, I>(self, _: I) -> Result<Self::Ok, Self::Error>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        unimplemented!()
    }

    fn collect_seq<I>(self, _: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        unimplemented!()
    }

    fn collect_str<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + std::fmt::Display,
    {
        unimplemented!()
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i128(self, _: i128) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_u128(self, _: u128) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

pub fn to_string<T>(name: &str, value: &T) -> Result<String, SirtSerializeError>
where
    T: Serialize,
{
    let mut s = SirtSerializer::new(name, false);
    value.serialize(&mut s)?;
    Ok(s.complete())
}

pub fn to_pretty_string<T>(name: &str, value: &T) -> Result<String, SirtSerializeError>
where
    T: Serialize,
{
    let mut s = SirtSerializer::new(name, true);
    value.serialize(&mut s)?;
    Ok(s.complete())
}
