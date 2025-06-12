fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.de.eat_char();

        let peek = match tri!(self.de.next_char()) {
            Some(b) => b,
            None => {
                return Err(self.de.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };

        let value = match peek {
            b't' => {
                tri!(self.de.parse_ident(b"rue\""));
                visitor.visit_bool(true)
            }
            b'f' => {
                tri!(self.de.parse_ident(b"alse\""));
                visitor.visit_bool(false)
            }
            _ => {
                self.de.scratch.clear();
                let s = tri!(self.de.read.parse_str(&mut self.de.scratch));
                Err(de::Error::invalid_type(Unexpected::Str(&s), &visitor))
            }
        };

        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.de.fix_position(err)),
        }
    }