use serde::Deserialize;
use serde::de::{
    self, DeserializeSeed, SeqAccess, Visitor
};
use crate::error::{Result, Error};

use crate::{parse_num, deserialize_macro, unimplemented_deserialize};

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
    input: &'de [u8],
    reader_pos: u32 // TODO: maybe do it a different way?
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer { input, reader_pos: 0}
    }
}

pub fn from_bytes<'a, T>(s: &'a [u8]) -> Result<T> where
    T: Deserialize<'a> {
        let mut deserializer = Deserializer::from_bytes(s);
        let t = T::deserialize(&mut deserializer)?;
        Ok(t)
}

impl<'de> Deserializer<'de> {
    parse_num!(peek_u16, parse_u16, u16, 2);
    parse_num!(peek_u32, parse_u32, u32, 4);
    parse_num!(peek_i32, parse_i32, i32, 4);

    fn parse_string(&mut self) -> Result<String> {
        let len = self.parse_i32()?;
        let res = String::from_utf8(self.input[self.reader_pos as usize .. (self.reader_pos + (len - 1) as u32) as usize].try_into().map_err(|_| Error::make_syntax())?).map_err(|_| Error::make_syntax())?;
        self.reader_pos += len as u32;
        Ok(res)
    }
    
    pub fn parsed_length(self) -> u32 {
        self.reader_pos
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    deserialize_macro!(deserialize_string, parse_string, visit_string);

    deserialize_macro!(deserialize_u16, parse_u16, visit_u16);
    deserialize_macro!(deserialize_i32, parse_i32, visit_i32);
    deserialize_macro!(deserialize_u32, parse_u32, visit_u32);

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
        let arr_len = self.parse_i32()?;
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