use std::io::{Cursor, Write};

use byteorder::{LittleEndian, WriteBytesExt};
use serde::{Serialize, ser};

use crate::error::{Result, Error};

pub struct Serializer<'se> {
    output: &'se mut Cursor<Vec<u8>>
}

pub fn to_vec<T>(value: &T) -> Result<Vec<u8>> 
where
    T: Serialize {
    let mut cursor = Cursor::new(Vec::new());
    let mut serializer = Serializer {
        output: &mut cursor
    };
    value.serialize(&mut serializer)?;
    Ok(cursor.get_ref().to_vec())
}

impl<'se, 'a> ser::Serializer for &'a mut Serializer<'se> {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    unimplemented_serialize!(serialize_i16, i16);
    unimplemented_serialize!(serialize_i64, i64);
    unimplemented_serialize!(serialize_u64, u64);
    unimplemented_serialize!(serialize_bool, bool);
    unimplemented_serialize!(serialize_i8, i8);
    unimplemented_serialize!(serialize_u8, u8);
    unimplemented_serialize!(serialize_f32, f32);
    unimplemented_serialize!(serialize_f64, f64);
    unimplemented_serialize!(serialize_char, char);
    unimplemented_serialize!(serialize_bytes, &[u8]);
    unimplemented_serialize!(serialize_unit_struct, &'static str);

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.output.write_i32::<LittleEndian>((v.len() as i32) + 1i32)?; // nullbyte
        self.output.write(v.as_bytes())?;
        self.output.write(&[0u8; 1])?; // nullbyte
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.output.write_i32::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.output.write_u16::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.output.write_u32::<LittleEndian>(v)?;
        Ok(())
    }

    
    fn serialize_unit(self) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        if let Some(len) = len {
            self.output.write_i32::<LittleEndian>(len as i32)?;
            Ok(self)
        }else {
            Err(Error::make_data(String::from("Length of the sequence must be known upfront!")))
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }
}

impl<'se, 'a> ser::SerializeSeq for &'a mut Serializer<'se> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'se, 'a> ser::SerializeTuple for &'a mut Serializer<'se> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'se, 'a> ser::SerializeTupleStruct for &'a mut Serializer<'se> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'se, 'a> ser::SerializeTupleVariant for &'a mut Serializer<'se> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

impl<'se, 'a> ser::SerializeMap for &'a mut Serializer<'se> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}


impl<'se, 'a> ser::SerializeStruct for &'a mut Serializer<'se> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'se, 'a> ser::SerializeStructVariant for &'a mut Serializer<'se> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}