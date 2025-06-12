// Answer 0

#[test]
fn test_parse_any_number_positive_zero() {
    let mut deserializer = Deserializer { 
        read: MyRead::new("0".as_bytes()), 
        scratch: Vec::new(), 
        remaining_depth: 10 
    };
    deserializer.parse_any_number(true);
}

#[test]
fn test_parse_any_number_positive_small() {
    let mut deserializer = Deserializer { 
        read: MyRead::new("12345".as_bytes()), 
        scratch: Vec::new(), 
        remaining_depth: 10 
    };
    deserializer.parse_any_number(true);
}

#[test]
fn test_parse_any_number_positive_large() {
    let mut deserializer = Deserializer { 
        read: MyRead::new("9999999999999999999".as_bytes()), 
        scratch: Vec::new(), 
        remaining_depth: 10 
    };
    deserializer.parse_any_number(true);
}

#[test]
fn test_parse_any_number_positive_overflow() {
    let mut deserializer = Deserializer { 
        read: MyRead::new("100000000000000000000".as_bytes()), 
        scratch: Vec::new(), 
        remaining_depth: 10 
    };
    deserializer.parse_any_number(true);
}

#[test]
fn test_parse_any_number_negative_zero() {
    let mut deserializer = Deserializer { 
        read: MyRead::new("0".as_bytes()), 
        scratch: Vec::new(), 
        remaining_depth: 10 
    };
    deserializer.parse_any_number(false);
}

#[test]
fn test_parse_any_number_negative_small() {
    let mut deserializer = Deserializer { 
        read: MyRead::new("-12345".as_bytes()), 
        scratch: Vec::new(), 
        remaining_depth: 10 
    };
    deserializer.parse_any_number(false);
}

#[test]
fn test_parse_any_number_negative_large() {
    let mut deserializer = Deserializer { 
        read: MyRead::new("-9999999999999999999".as_bytes()), 
        scratch: Vec::new(), 
        remaining_depth: 10 
    };
    deserializer.parse_any_number(false);
}

#[test]
fn test_parse_any_number_negative_overflow() {
    let mut deserializer = Deserializer { 
        read: MyRead::new("-100000000000000000000".as_bytes()), 
        scratch: Vec::new(), 
        remaining_depth: 10 
    };
    deserializer.parse_any_number(false);
}

#[test]
#[should_panic]
fn test_parse_any_number_invalid_character() {
    let mut deserializer = Deserializer { 
        read: MyRead::new("abc".as_bytes()), 
        scratch: Vec::new(), 
        remaining_depth: 10 
    };
    deserializer.parse_any_number(true);
}

#[test]
#[should_panic]
fn test_parse_any_number_positive_invalid_overflow() {
    let mut deserializer = Deserializer { 
        read: MyRead::new("100000000000000000001".as_bytes()), 
        scratch: Vec::new(), 
        remaining_depth: 10 
    };
    deserializer.parse_any_number(true);
}

#[test]
#[should_panic]
fn test_parse_any_number_negative_invalid_overflow() {
    let mut deserializer = Deserializer { 
        read: MyRead::new("-100000000000000000001".as_bytes()), 
        scratch: Vec::new(), 
        remaining_depth: 10 
    };
    deserializer.parse_any_number(false);
}


struct MyRead<'a> {
    data: &'a [u8],
    index: usize,
}

impl<'a> MyRead<'a> {
    fn new(data: &'a [u8]) -> Self {
        MyRead { data, index: 0 }
    }
}

impl<'de> Read<'de> for MyRead<'de> {
    const should_early_return_if_failed: bool = false;

    fn next(&mut self) -> Result<Option<u8>> {
        if self.index < self.data.len() {
            let byte = self.data[self.index];
            self.index += 1;
            Ok(Some(byte))
        } else {
            Ok(None)
        }
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        if self.index < self.data.len() {
            Ok(Some(self.data[self.index]))
        } else {
            Ok(None)
        }
    }

    fn discard(&mut self) {
        self.index += 1;
    }

    fn position(&self) -> Position {
        Position::from(self.index)
    }
    
    fn peek_position(&self) -> Position {
        Position::from(self.index)
    }

    fn byte_offset(&self) -> usize {
        self.index
    }

    fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
        unimplemented!()
    }
    
    fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
        unimplemented!()
    }
    
    fn ignore_str(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn decode_hex_escape(&mut self) -> Result<u16> {
        unimplemented!()
    }
    
    fn set_failed(&mut self, _failed: &mut bool) {
        unimplemented!()
    }
}

