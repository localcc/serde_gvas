#[macro_export]
macro_rules! parse_num {
    ($method:ident, $num:ty, $size:literal) => {
        fn $method(&mut self) -> Result<$num> {
            let val = <$num>::from_le_bytes(self.input[self.reader_pos as usize .. (self.reader_pos + $size) as usize].try_into().map_err(|_| Error::make_syntax())?);
            self.reader_pos += $size;
            Ok(val)
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
    }
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
