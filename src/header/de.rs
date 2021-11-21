use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};
use serde::Deserialize;
use serde::de::{
    self, DeserializeSeed, SeqAccess, Visitor
};
use crate::error::{Result, Error};

use crate::{unimplemented_deserialize};

struct ArrayAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    elements: i32
}

impl<'a, 'de: 'a> ArrayAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, elements: i32) -> Self {
        ArrayAccess {de, elements}
    }
}

impl<'de, 'a> SeqAccess<'de> for ArrayAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>> where
        T: DeserializeSeed<'de> 
    {
        if self.elements == 0 {
            return Ok(None);
        }
        self.elements -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }
}

pub struct Deserializer<'de> {
    input: &'de mut Cursor<Vec<u8>>,
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de mut Cursor<Vec<u8>>) -> Self {
        Deserializer { input }
    }
}

pub fn from_bytes<'a, T>(s: &'a mut Cursor<Vec<u8>>) -> Result<T> where
    T: Deserialize<'a> {
        let mut deserializer = Deserializer::from_bytes(s);
        let t = T::deserialize(&mut deserializer)?;
        Ok(t)
}

impl<'de> Deserializer<'de> {
    fn parse_string(&mut self) -> Result<String> {
        let len = self.input.read_i32::<LittleEndian>()?;
        let mut buf = vec![0u8; len as usize];
        self.input.read_exact(&mut buf)?;
        let mut s = String::from_utf8(buf)?;
        s.pop(); // nullbyte
        Ok(s)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    deserialize_macro!(deserialize_string, parse_string, visit_string);

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
            V: Visitor<'de> {

        visitor.visit_u16(self.input.read_u16::<LittleEndian>()?)
    }
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
            V: Visitor<'de> {
        visitor.visit_i32(self.input.read_i32::<LittleEndian>()?)
    }
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
            V: Visitor<'de> {
                
        visitor.visit_u32(self.input.read_u32::<LittleEndian>()?)
    }

    unimplemented_deserialize!(deserialize_any, deserialize_i16, deserialize_i64, deserialize_u64, deserialize_bool, deserialize_i8, deserialize_u8, deserialize_f32, deserialize_f64, deserialize_char, 
        deserialize_str, deserialize_bytes, deserialize_byte_buf, deserialize_option, deserialize_unit, deserialize_map, deserialize_identifier, deserialize_ignored_any);
    
    fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<V::Value> where
        V: Visitor<'de> 
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, _: V) -> Result<V::Value> where
        V: Visitor<'de> 
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value> where
        V: Visitor<'de> 
    {
        let arr_len = self.input.read_i32::<LittleEndian>()?;
        visitor.visit_seq(ArrayAccess::new(self, arr_len))
    }

    fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value> where
        V: Visitor<'de> 
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(self, _: &'static str, _: usize, _: V) -> Result<V::Value> where
        V: Visitor<'de> 
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(self, _: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value> where
        V: Visitor<'de> {
        visitor.visit_seq(ArrayAccess::new(self, fields.len() as i32))
    }

    fn deserialize_enum<V>(self, _: &'static str, _: &'static [&'static str], _: V) -> Result<V::Value> where
        V: Visitor<'de> 
    {
        unimplemented!()
    }

}