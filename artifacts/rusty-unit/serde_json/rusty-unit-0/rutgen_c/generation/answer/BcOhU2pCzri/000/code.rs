// Answer 0

#[test]
fn test_parse_str_raw_success() {
    use std::io::Cursor;
    use crate::error::{Error, Result};
    
    struct MockReader {
        data: Cursor<Vec<u8>>,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self {
                data: Cursor::new(data),
            }
        }
    }

    #[cfg(feature = "std")]
    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            let mut buf = [0; 1];
            match self.data.read(&mut buf) {
                Ok(0) => Ok(None),
                Ok(_) => Ok(Some(buf[0])),
                Err(_) => Err(Error::from("Read error")),
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            let pos = self.data.position();
            let result = self.next()?;
            self.data.set_position(pos);
            result
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { unimplemented!() }

        fn peek_position(&self) -> Position { unimplemented!() }

        fn byte_offset(&self) -> usize { unimplemented!() }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }

        fn ignore_str(&mut self) -> Result<()> { 

            unimplemented!() 
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let test_data = b"test data".to_vec();
    let mut scratch = vec![];
    let mut reader = MockReader::new(test_data);
    
    let result = reader.parse_str_raw(&mut scratch);
    
    assert!(result.is_ok());
}

#[test]
fn test_parse_str_raw_empty_input() {
    use std::io::Cursor;
    use crate::error::{Error, Result};

    struct MockReader {
        data: Cursor<Vec<u8>>,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self {
                data: Cursor::new(data),
            }
        }
    }

    #[cfg(feature = "std")]
    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            let mut buf = [0; 1];
            match self.data.read(&mut buf) {
                Ok(0) => Ok(None),
                Ok(_) => Ok(Some(buf[0])),
                Err(_) => Err(Error::from("Read error")),
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            let pos = self.data.position();
            let result = self.next()?;
            self.data.set_position(pos);
            result
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position { unimplemented!() }

        fn peek_position(&self) -> Position { unimplemented!() }

        fn byte_offset(&self) -> usize { unimplemented!() }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> { unimplemented!() }

        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
    }

    let test_data = b"".to_vec();
    let mut scratch = vec![];
    let mut reader = MockReader::new(test_data);

    let result = reader.parse_str_raw(&mut scratch);
    
    assert!(result.is_err());
}

