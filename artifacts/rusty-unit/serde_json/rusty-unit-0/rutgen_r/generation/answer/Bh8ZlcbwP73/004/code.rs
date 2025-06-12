// Answer 0

#[test]
fn test_parse_str_bytes_fast_path_no_escape() {
    struct SliceReader {
        slice: &'static [u8],
        index: usize,
    }

    impl SliceReader {
        fn skip_to_escape(&mut self, _validate: bool) {
            while self.index < self.slice.len() && self.slice[self.index] != b'\\' {
                self.index += 1;
            }
        }
    }

    let mut reader = SliceReader {
        slice: b"hello world\"",
        index: 0,
    };
    let mut scratch = Vec::new();

    let result = reader.parse_str_bytes(&mut scratch, false, |_, data| {
        Ok(data as &[u8])
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"hello world");
}

#[test]
fn test_parse_str_bytes_with_escape() {
    struct SliceReader {
        slice: &'static [u8],
        index: usize,
    }

    impl SliceReader {
        fn skip_to_escape(&mut self, _validate: bool) {
            while self.index < self.slice.len() && self.slice[self.index] != b'\\' {
                self.index += 1;
            }
        }

        fn parse_str_bytes<'s, T, F>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
            validate: bool,
            result: F,
        ) -> Result<&'s [u8], &'static str>
        where
            T: ?Sized + 's,
            F: for<'f> FnOnce(&'s Self, &'f [u8]) -> Result<&'f T>,
        {
            let mut start = self.index;

            loop {
                self.skip_to_escape(validate);
                if self.index == self.slice.len() {
                    return Err("EOF While Parsing String");
                }
                match self.slice[self.index] {
                    b'"' => {
                        if scratch.is_empty() {
                            let borrowed = &self.slice[start..self.index];
                            self.index += 1;
                            return result(self, borrowed).map_err(|_| "Error");
                        } else {
                            scratch.extend_from_slice(&self.slice[start..self.index]);
                            self.index += 1;
                            return result(self, scratch).map_err(|_| "Error");
                        }
                    }
                    b'\\' => {
                        scratch.extend_from_slice(&self.slice[start..self.index]);
                        self.index += 1;
                        // Assuming parse_escape can be called directly here.
                        // For testing purposes, returning Ok.
                        let _ = Ok(());
                        start = self.index;
                    }
                    _ => {
                        self.index += 1;
                        return Err("Control Character While Parsing String");
                    }
                }
            }
        }
    }

    let mut reader = SliceReader {
        slice: b"hello \\\"world\"",
        index: 0,
    };
    let mut scratch = Vec::new();

    let result = reader.parse_str_bytes(&mut scratch, true, |_, data| {
        Ok(data as &[u8])
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"hello \"world");
}

#[test]
fn test_parse_str_bytes_control_character() {
    struct SliceReader {
        slice: &'static [u8],
        index: usize,
    }

    impl SliceReader {
        fn skip_to_escape(&mut self, _validate: bool) {
            while self.index < self.slice.len() && self.slice[self.index] != b'\\' {
                self.index += 1;
            }
        }

        fn parse_str_bytes<'s, T, F>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
            validate: bool,
            result: F,
        ) -> Result<&'s [u8], &'static str>
        where
            T: ?Sized + 's,
            F: for<'f> FnOnce(&'s Self, &'f [u8]) -> Result<&'f T>,
        {
            let mut start = self.index;

            loop {
                self.skip_to_escape(validate);
                if self.index == self.slice.len() {
                    return Err("EOF While Parsing String");
                }
                match self.slice[self.index] {
                    b'"' => {
                        if scratch.is_empty() {
                            let borrowed = &self.slice[start..self.index];
                            self.index += 1;
                            return result(self, borrowed).map_err(|_| "Error");
                        } else {
                            scratch.extend_from_slice(&self.slice[start..self.index]);
                            self.index += 1;
                            return result(self, scratch).map_err(|_| "Error");
                        }
                    }
                    b'\\' => {
                        scratch.extend_from_slice(&self.slice[start..self.index]);
                        self.index += 1;
                        let _ = Ok(());
                        start = self.index;
                    }
                    _ => {
                        self.index += 1;
                        return Err("Control Character While Parsing String");
                    }
                }
            }
        }
    }

    let mut reader = SliceReader {
        slice: b"hello \x01world\"",
        index: 0,
    };
    let mut scratch = Vec::new();

    let result = reader.parse_str_bytes(&mut scratch, true, |_, _| {
        Err("Invalid Data")
    });

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Control Character While Parsing String");
}

