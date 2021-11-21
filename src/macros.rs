#[macro_export]
macro_rules! parse_num {
    ($peek:ident, $method:ident, $read_func:ident) => {
        fn $peek(&mut self) -> Result<$num> {
            Ok(self.input.<$read_func>::<LittleEndian>().map_err(|_| Error::make_syntax())?)
        }
        fn $method(&mut self) -> Result<$num> {
            let num = self.$peek()?;
            self.reader_pos += $size;
            Ok(num)
        }
    };
}

#[macro_export]
macro_rules! deserialize_macro {
    ($method:ident, $call_method:ident, $visit:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value> where
            V: Visitor<'de> {
            visitor.$visit(self.$call_method()?)
        }
    };

}

#[macro_export]
macro_rules! unimplemented_deserialize {
    ($($method:ident),*) => {
        $(fn $method<V>(self, _: V) -> Result<V::Value> where
            V: Visitor<'de> {
                unimplemented!()
            })*
    };
}

#[macro_export]
macro_rules! unimplemented_serialize {
    ($method:ident,$type:ty) => {
        fn $method(self, _: $type) -> Result<Self::Ok> {
            unimplemented!()
        }
    };
}

#[macro_export]
macro_rules! parse_number_property {
    ($method:ident, $read_method:ident, $endianness:ty, $num:ty, $size:literal) => {
        fn $method(&mut self) -> Result<$num> {
            let value_size = self.input.read_i64::<LittleEndian>()?;
            if value_size != $size {
                return Err(Error::make_other(format!("Expected value size of {} got {} at position {}", $size, value_size, self.input.position())));
            }
            self.input.read_exact(&mut [0u8; 1])?;
            Ok(self.input.$read_method::<$endianness>()?)
        }
    };

    ($method:ident, $read_method:ident, $num:ty, $size:literal) => {
        fn $method(&mut self) -> Result<$num> {
            let value_size = self.input.read_i64::<LittleEndian>()?;
            if value_size != $size {
                return Err(Error::make_other(format!("Expected value size of {} got {} at position {}", $size, value_size, self.input.position())));
            }
            self.input.read_exact(&mut [0u8; 1])?;
            Ok(self.input.$read_method()?)
        }
    }
}