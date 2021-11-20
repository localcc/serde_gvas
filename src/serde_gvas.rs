use serde::{Deserialize, de};
use serde::de::{DeserializeSeed, SeqAccess, Visitor};

use crate::error::{Result, Error};
use crate::types::FGuid;
use crate::{parse_num, unimplemented_deserialize};
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

struct MapKey<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>
}

impl<'a, 'de> de::Deserializer<'de> for MapKey<'a, 'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
            V: Visitor<'de> {
        visitor.visit_string(self.de.peek_string()?)
    }

    serde::forward_to_deserialize_any! {
        bool i8 u8 i16 u16 i32 u32 i64 u64 f32 f64 char str
        string unit unit_struct seq tuple tuple_struct map
        struct identifier ignored_any bytes byte_buf option
        newtype_struct enum
    }
}

struct MapAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> MapAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        MapAccess { de }
    }
}

impl<'de, 'a> serde::de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de> {
        let value_name = self.de.peek_string()?;
        if value_name == "None" {
            return Ok(None);
        }
        seed.deserialize(MapKey {de: &mut *self.de}).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de> {
        seed.deserialize(&mut *self.de)
    }
}


pub struct Deserializer<'de> {
    input: &'de [u8],
    reader_pos: u32 // TODO: maybe do it a different way?
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8], reader_pos: u32) -> Self {
        Deserializer { input, reader_pos }
    }
}

pub fn from_bytes<'a, T>(s: &'a [u8], header_length: u32) -> Result<T> where
    T: Deserialize<'a> {
        let mut deserializer = Deserializer::from_bytes(s, header_length);
        let t = T::deserialize(&mut deserializer)?;
        Ok(t)
}

macro_rules! parse_number_property {
    ($method:ident, $underlying_method:ident, $num:ty, $val_type:literal, $size:literal) => {
        fn $method(&mut self) -> Result<$num> {
            let value_size = self.parse_i64()?;
            if value_size != $size {
                return Err(Error::make_syntax());
            }
            self.reader_pos += 1; // terminator byte
            self.$underlying_method()
        }
    };
}

impl<'de> Deserializer<'de> {
    parse_num!(peek_i8, parse_i8, i8, 1);
    parse_num!(peek_u8, parse_u8, u8, 1);
    parse_num!(peek_i16, parse_i16, i16, 2);
    parse_num!(peek_u16, parse_u16, u16, 2);
    parse_num!(peek_i32, parse_i32, i32, 4);
    parse_num!(peek_u32, parse_u32, u32, 4);
    parse_num!(peek_i64, parse_i64, i64, 8);
    parse_num!(peek_u64, parse_u64, u64, 8);
    parse_num!(peek_f32, parse_f32, f32, 4);
    parse_num!(peek_f64, parse_f64, f64, 8);

    fn parse_u8_property(&mut self) -> Result<u8> {
        let value_size = self.parse_i64()?;
        if value_size != 1 {
            return Err(Error::make_syntax());
        }

        // this is unique for byteproperty it seems?
        let _unk = self.parse_string()?; // todo: what is that even for? enum name?
        self.reader_pos += 1; // skipping another byte for some reason

        self.parse_u8()
    }

    fn parse_bool_property(&mut self) -> Result<bool> {
        let value_size = self.parse_i64()?;
        // 0 for some reason on boolproperty
        if value_size != 0 {
            return Err(Error::make_syntax());
        }
        
        let val = self.parse_u16()?;
        Ok(val > 0)
    }

    parse_number_property!(parse_i8_property, parse_i8, i8, "Int8Property", 1);
    parse_number_property!(parse_i16_property, parse_i16, i16, "Int16Property", 2);
    parse_number_property!(parse_u16_property, parse_u16, u16, "UInt16Property", 2);
    parse_number_property!(parse_i32_property, parse_i32, i32, "IntProperty", 4);
    parse_number_property!(parse_u32_property, parse_u32, u32, "UInt32Property", 4);
    parse_number_property!(parse_i64_property, parse_i64, i64, "Int64Property", 8);
    parse_number_property!(parse_u64_property, parse_u64, u64, "UInt64Property", 8);
    parse_number_property!(parse_f32_property, parse_f32, f32, "FloatProperty", 4);
    parse_number_property!(parse_f64_property, parse_f64, f64, "DoubleProperty", 8);

    fn peek_string(&mut self) -> Result<String> {
        let len = self.peek_i32()?;
        if len > self.input.len() as i32 {
           return Err(Error::make_syntax());
        }
        // -1 because nullbyte
        let res = String::from_utf8(self.input[(self.reader_pos + 4) as usize .. ((self.reader_pos + 4) + (len - 1) as u32) as usize].try_into().map_err(|_| Error::make_syntax())?).map_err(|_| Error::make_syntax())?;
        Ok(res)
    }
        
    fn parse_string(&mut self) -> Result<String> {
        let len = self.parse_i32()?;
        if len > self.input.len() as i32 {
            return Err(Error::make_syntax());
        }
        // -1 because nullbyte
        let res = String::from_utf8(self.input[self.reader_pos as usize .. (self.reader_pos + (len - 1) as u32) as usize].try_into().map_err(|_| Error::make_syntax())?).map_err(|_| Error::make_syntax())?;
        self.reader_pos += len as u32;
        Ok(res)
    }

    fn parse_string_property(&mut self) -> Result<String> {
        let _value_size = self.parse_i64()?;
        self.reader_pos += 1; // terminator byte

        self.parse_string()
    }

    fn parse_guid(&mut self) -> Result<FGuid> {
        let guid = FGuid::new(self.parse_u32()?, self.parse_u32()?, self.parse_u32()?, self.parse_u32()?);
        self.reader_pos += 1; // terminator byte
        Ok(guid)
    }

    fn deserialize_struct<V>(&mut self, visitor: V) -> Result<V::Value> 
    where
        V: Visitor<'de> {
        let _struct_value_len = self.parse_i64()?;
        let value_type = self.parse_string()?;
        let _struct_guid = self.parse_guid()?;

        match value_type.as_str() {
            "DateTime" => visitor.visit_u64(self.parse_u64()?),
            _ => visitor.visit_map(MapAccess::new(self))
        }
    }
}


impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    serde::forward_to_deserialize_any! {
        i8 u8 i16 u16 i32 u32 i64 u64 f32 f64 bool string
    }

    unimplemented_deserialize!(deserialize_char, 
        deserialize_str, deserialize_bytes, deserialize_byte_buf, deserialize_option, deserialize_unit, deserialize_map, deserialize_identifier);
    
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
            V: Visitor<'de> {
        self.deserialize_any(visitor)
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
            V: Visitor<'de> {
        let _value_name = self.parse_string()?;
        let value_type = self.parse_string()?;
        
        match value_type.as_str() {
            "StructProperty" => self.deserialize_struct(visitor),
            "Int8Property" => visitor.visit_i8(self.parse_i8_property()?),
            "ByteProperty" => visitor.visit_u8(self.parse_u8_property()?),
            "Int16Property" => visitor.visit_i16(self.parse_i16_property()?),
            "UInt16Property" => visitor.visit_u16(self.parse_u16_property()?),
            "IntProperty" => visitor.visit_i32(self.parse_i32_property()?),
            "UInt32Property" => visitor.visit_u32(self.parse_u32_property()?),
            "Int64Property" => visitor.visit_i64(self.parse_i64_property()?),
            "UInt64Property" => visitor.visit_u64(self.parse_u64_property()?),
            "FloatProperty" => visitor.visit_f32(self.parse_f32_property()?),
            "DoubleProperty" => visitor.visit_f64(self.parse_f64_property()?),
            "StrProperty" => visitor.visit_string(self.parse_string_property()?),
            "BoolProperty" => visitor.visit_bool(self.parse_bool_property()?),
            _ => Err(Error::make_syntax())
        }
    }

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

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<V::Value> where
        V: Visitor<'de> {
            return visitor.visit_map(MapAccess::new(self));
    }

    fn deserialize_enum<V>(self, _: &'static str, _: &'static [&'static str], _: V) -> Result<V::Value> where
        V: Visitor<'de> 
    {
        unimplemented!()
    }

}