pub(crate) fn do_deserialize_u128<'any, V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'any>,
    {
        match tri!(self.parse_whitespace()) {
            Some(b'-') => {
                return Err(self.peek_error(ErrorCode::NumberOutOfRange));
            }
            Some(_) => {}
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        }

        let mut buf = String::new();
        tri!(self.scan_integer128(&mut buf));

        let value = match buf.parse() {
            Ok(int) => visitor.visit_u128(int),
            Err(_) => {
                return Err(self.error(ErrorCode::NumberOutOfRange));
            }
        };

        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }