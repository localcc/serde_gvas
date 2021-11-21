use std::io::{Cursor, Read};
use std::marker::PhantomData;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Deserialize, de};
use serde::de::{DeserializeSeed, SeqAccess, Visitor, value};

use crate::error::{Result, Error};
use crate::types::FGuid;
use crate::{parse_num, unimplemented_deserialize};

struct ArrayAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    elements: i32,
    t: String,
    struct_type: Option<String>
}


impl<'a, 'de: 'a> ArrayAccess<'a, 'de> {
    pub fn new(de: &'a mut Deserializer<'de>, elements: i32, t: String, struct_type: Option<String>) -> Self {
        ArrayAccess {
            de,
            elements,
            t,
            struct_type
        }
    }
}

impl<'a, 'de: 'a> SeqAccess<'de> for ArrayAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de> {
        if self.elements == 0 {
            return Ok(None);
        }
        self.elements -= 1;

        let mut ad = ArrayDeserializer::new(&mut *self.de, &self.t, &self.struct_type);
        seed.deserialize(&mut ad).map(Some)
    }
}

struct MapKey<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>
}

impl<'a, 'de> de::Deserializer<'de> for MapKey<'a, 'de> {
    type Error = Error;
// 
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
    i: i32
}

impl<'a, 'de> MapAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, i: i32) -> Self {
        MapAccess { de, i }
    }
}

impl<'de, 'a> serde::de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de> {
        let value_name = self.de.peek_string()?;
        if value_name == "None" {
            self.de.parse_string()?;
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

struct ArrayDeserializer<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    t: &'a String,
    struct_type: &'a Option<String>
}

impl<'a, 'de> ArrayDeserializer<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, t: &'a String, struct_type: &'a Option<String>) -> Self {
        ArrayDeserializer { de, t, struct_type } 
    }

    fn deserialize_struct<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de> {
        if let Some(struct_type) = &self.struct_type {
            match struct_type.as_str() {
                "DateTime" => visitor.visit_u64(self.de.input.read_u64::<LittleEndian>()?),
                _ => visitor.visit_map(MapAccess::new(&mut *self.de, -1))
            }
        } else {
            Err(Error::make_other(String::from("Trying to deserialize struct without specifying the type!")))
        }
    }
}


impl<'de, 'a> de::Deserializer<'de> for &'a mut ArrayDeserializer<'a, 'de> {
    type Error = Error;

    serde::forward_to_deserialize_any! {
        i8 u8 i16 u16 i32 u32 i64 u64 f32 f64 bool string ignored_any
    }

    unimplemented_deserialize!(deserialize_char, 
        deserialize_str, deserialize_bytes, deserialize_byte_buf, deserialize_option, deserialize_unit, deserialize_map, deserialize_identifier);

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de> {
        match self.t.as_str() {
            "StructProperty" => self.deserialize_struct(visitor),
            "Int8Property" => visitor.visit_i8(self.de.input.read_i8()?),
            "ByteProperty" => visitor.visit_u8(self.de.input.read_u8()?),
            "Int16Property" => visitor.visit_i16(self.de.input.read_i16::<LittleEndian>()?),
            "UInt16Property" => visitor.visit_u16(self.de.input.read_u16::<LittleEndian>()?),
            "IntProperty" => visitor.visit_i32(self.de.input.read_i32::<LittleEndian>()?),
            "UInt32Property" => visitor.visit_u32(self.de.input.read_u32::<LittleEndian>()?),
            "Int64Property" => visitor.visit_i64(self.de.input.read_i64::<LittleEndian>()?),
            "UInt64Property" => visitor.visit_u64(self.de.input.read_u64::<LittleEndian>()?),
            "FloatProperty" => visitor.visit_f32(self.de.input.read_f32::<LittleEndian>()?),
            "DoubleProperty" => visitor.visit_f64(self.de.input.read_f64::<LittleEndian>()?),
            "StrProperty" => visitor.visit_string(self.de.parse_string()?),
            "BoolProperty" => visitor.visit_bool(self.de.input.read_u8()? == 1),
            _ => Err(Error::make_other(format!("Unkown property type {}", self.t)))
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
        unimplemented!()
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
            return visitor.visit_map(MapAccess::new(&mut *self.de, -1));
    }

    fn deserialize_enum<V>(self, _: &'static str, _: &'static [&'static str], _: V) -> Result<V::Value> where
        V: Visitor<'de> 
    {
        unimplemented!()
    }

}

pub struct Deserializer<'de> {
    input: &'de mut Cursor<Vec<u8>>,
    first: bool
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de mut Cursor<Vec<u8>>) -> Self {
        Deserializer { input, first: true }
    }
}

pub fn from_bytes<'a, T>(input: &'a mut Cursor<Vec<u8>>) -> Result<T> where
    T: Deserialize<'a> {
        let mut deserializer = Deserializer::from_bytes(input);
        let t = T::deserialize(&mut deserializer)?;
        Ok(t)
}


impl<'de> Deserializer<'de> {

    fn parse_u8_property(&mut self) -> Result<u8> {
        let value_size = self.input.read_i64::<LittleEndian>()?;
        if value_size != 1 {
            return Err(Error::make_other(format!("Expected value size of 1 got {} at position {}", value_size, self.input.position())));
        }

        // this is unique for byteproperty it seems?
        let _unk = self.parse_string()?; // todo: what is that even for? enum name?
        self.input.read_exact(&mut [0u8; 1])?; // skipping another byte for some reason

        Ok(self.input.read_u8()?)
    }

    fn parse_bool_property(&mut self) -> Result<bool> {
        let value_size = self.input.read_i64::<LittleEndian>()?;
        // 0 for some reason on boolproperty
        if value_size != 0 {
            return Err(Error::make_other(format!("Expected value size of 0 got {} at position {}", value_size, self.input.position())));
        }
        
        let val = self.input.read_u16::<LittleEndian>()?;
        Ok(val > 0)
    }

    parse_number_property!(parse_i8_property, read_i8, i8, 1);
    parse_number_property!(parse_i16_property, read_i16, LittleEndian, i16, 2);
    parse_number_property!(parse_u16_property, read_u16, LittleEndian, u16, 2);
    parse_number_property!(parse_i32_property, read_i32, LittleEndian, i32, 4);
    parse_number_property!(parse_u32_property, read_u32, LittleEndian, u32, 4);
    parse_number_property!(parse_i64_property, read_i64, LittleEndian, i64, 8);
    parse_number_property!(parse_u64_property, read_u64, LittleEndian, u64, 8);
    parse_number_property!(parse_f32_property, read_f32, LittleEndian, f32, 4);
    parse_number_property!(parse_f64_property, read_f64, LittleEndian, f64, 8);
    
    fn peek_string(&mut self) -> Result<String> {
        let pos = self.input.position();
        let s = self.parse_string()?;
        self.input.set_position(pos);
        Ok(s)
    }
        
    fn parse_string(&mut self) -> Result<String> {
        //todo: maybe length checking?
        let len = self.input.read_i32::<LittleEndian>()?;
        
        let mut str_bytes = vec![0u8; len as usize];
        self.input.read_exact(&mut str_bytes)?;
        let mut s = String::from_utf8(str_bytes)?;
        s.pop(); // nullbyte
        Ok(s)
    }

    fn parse_string_property(&mut self) -> Result<String> {
        let _value_size = self.input.read_i64::<LittleEndian>()?;
        self.input.read_exact(&mut [0u8; 1])?;

        self.parse_string()
    }

    fn parse_guid(&mut self) -> Result<FGuid> {
        let guid = FGuid::new(self.input.read_u32::<LittleEndian>()?, self.input.read_u32::<LittleEndian>()?, self.input.read_u32::<LittleEndian>()?, self.input.read_u32::<LittleEndian>()?);
        Ok(guid)
    }

    fn deserialize_struct<V>(&mut self, visitor: V) -> Result<V::Value> 
    where
        V: Visitor<'de> {
        let _struct_value_len = self.input.read_i64::<LittleEndian>()?;
        let value_type = self.parse_string()?;
        let _struct_guid = self.parse_guid()?;
        self.input.read_exact(&mut [0u8; 1])?; // terminator

        match value_type.as_str() {
            "DateTime" => visitor.visit_u64(self.input.read_u64::<LittleEndian>()?),
            _ => visitor.visit_map(MapAccess::new(self, 1))
        }
    }

    fn deserialize_array<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de> {
        let struct_value_len = self.input.read_i64::<LittleEndian>()?;
        let value_type = self.parse_string()?;

        self.input.read_exact(&mut [0u8; 1])?;
        let elements = match value_type.as_str() {
            "StructProperty" => {
                let len = self.input.read_i32::<LittleEndian>()?;

                let dup_var_name = self.parse_string()?;
                let dup_type_name = self.parse_string()?;
                let dup_value_len = self.input.read_i64::<LittleEndian>()?;
                let struct_name = self.parse_string()?;
                let _struct_guid = self.parse_guid()?;

                self.input.read_exact(&mut [0u8; 1])?; //terminator
                len
            },
            _ => self.input.read_i32::<LittleEndian>()?
        };
        visitor.visit_seq(ArrayAccess::new(self, elements, value_type, Some(String::from("TestStruct"))))
    }
}


impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    serde::forward_to_deserialize_any! {
        i8 u8 i16 u16 i32 u32 i64 u64 f32 f64 bool string seq
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
        let value_name = self.parse_string()?;
        
        let value_type = self.parse_string()?;
        match value_type.as_str() {
            "StructProperty" => self.deserialize_struct(visitor),
            "ArrayProperty" => self.deserialize_array(visitor),
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
            _ => Err(Error::make_other(format!("Unknown property type {}", value_type)))
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
            if self.first {
                self.first = false;
                return visitor.visit_map(MapAccess::new(self, 0));
            }else {
                return self.deserialize_any(visitor);
            }
    }

    fn deserialize_enum<V>(self, _: &'static str, _: &'static [&'static str], _: V) -> Result<V::Value> where
        V: Visitor<'de> 
    {
        unimplemented!()
    }

}