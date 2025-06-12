fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match tri!(self.parse_whitespace()) {
            Some(b'{') => {
                check_recursion! {
                    self.eat_char();
                    let ret = visitor.visit_enum(VariantAccess::new(self));
                }
                let value = tri!(ret);

                match tri!(self.parse_whitespace()) {
                    Some(b'}') => {
                        self.eat_char();
                        Ok(value)
                    }
                    Some(_) => Err(self.error(ErrorCode::ExpectedSomeValue)),
                    None => Err(self.error(ErrorCode::EofWhileParsingObject)),
                }
            }
            Some(b'"') => visitor.visit_enum(UnitVariantAccess::new(self)),
            Some(_) => Err(self.peek_error(ErrorCode::ExpectedSomeValue)),
            None => Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
        }
    }