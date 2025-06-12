fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        fn has_next_element<'de, 'a, R: Read<'de> + 'a>(
            seq: &mut SeqAccess<'a, R>,
        ) -> Result<bool> {
            let peek = match tri!(seq.de.parse_whitespace()) {
                Some(b) => b,
                None => {
                    return Err(seq.de.peek_error(ErrorCode::EofWhileParsingList));
                }
            };

            if peek == b']' {
                Ok(false)
            } else if seq.first {
                seq.first = false;
                Ok(true)
            } else if peek == b',' {
                seq.de.eat_char();
                match tri!(seq.de.parse_whitespace()) {
                    Some(b']') => Err(seq.de.peek_error(ErrorCode::TrailingComma)),
                    Some(_) => Ok(true),
                    None => Err(seq.de.peek_error(ErrorCode::EofWhileParsingValue)),
                }
            } else {
                Err(seq.de.peek_error(ErrorCode::ExpectedListCommaOrEnd))
            }
        }

        if tri!(has_next_element(self)) {
            Ok(Some(tri!(seed.deserialize(&mut *self.de))))
        } else {
            Ok(None)
        }
    }