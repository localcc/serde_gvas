#[macro_export]
macro_rules! parse_num {
    ($peek:ident, $method:ident, $num:ty, $size:literal) => {
        fn $peek(&mut self) -> Result<$num> {
            Ok(<$num>::from_le_bytes(self.input[self.reader_pos as usize .. (self.reader_pos + $size) as usize].try_into().map_err(|_| Error::make_syntax())?))
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
