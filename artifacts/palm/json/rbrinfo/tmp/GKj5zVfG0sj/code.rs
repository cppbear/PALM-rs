fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
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
            b'[' => {
                check_recursion! {
                    self.eat_char();
                    let ret = visitor.visit_seq(SeqAccess::new(self));
                }

                match (ret, self.end_seq()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            b'{' => {
                check_recursion! {
                    self.eat_char();
                    let ret = visitor.visit_map(MapAccess::new(self));
                }

                match (ret, self.end_map()) {
                    (Ok(ret), Ok(())) => Ok(ret),
                    (Err(err), _) | (_, Err(err)) => Err(err),
                }
            }
            _ => Err(self.peek_invalid_type(&visitor)),
        };

        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }