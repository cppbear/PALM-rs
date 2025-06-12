fn next(&mut self) -> Option<Result<T>> {
        if R::should_early_return_if_failed && self.failed {
            return None;
        }

        // skip whitespaces, if any
        // this helps with trailing whitespaces, since whitespaces between
        // values are handled for us.
        match self.de.parse_whitespace() {
            Ok(None) => {
                self.offset = self.de.read.byte_offset();
                None
            }
            Ok(Some(b)) => {
                // If the value does not have a clear way to show the end of the value
                // (like numbers, null, true etc.) we have to look for whitespace or
                // the beginning of a self-delineated value.
                let self_delineated_value = match b {
                    b'[' | b'"' | b'{' => true,
                    _ => false,
                };
                self.offset = self.de.read.byte_offset();
                let result = de::Deserialize::deserialize(&mut self.de);

                Some(match result {
                    Ok(value) => {
                        self.offset = self.de.read.byte_offset();
                        if self_delineated_value {
                            Ok(value)
                        } else {
                            self.peek_end_of_value().map(|()| value)
                        }
                    }
                    Err(e) => {
                        self.de.read.set_failed(&mut self.failed);
                        Err(e)
                    }
                })
            }
            Err(e) => {
                self.de.read.set_failed(&mut self.failed);
                Some(Err(e))
            }
        }
    }