fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let peek = match tri!(self.parse_whitespace()) {
            Some(b) => b,
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };

        let value = match peek {
            b'n' => {
                self.eat_char();
                tri!(self.parse_ident(b"ull"));
                visitor.visit_unit()
            }
            _ => Err(self.peek_invalid_type(&visitor)),
        };

        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }