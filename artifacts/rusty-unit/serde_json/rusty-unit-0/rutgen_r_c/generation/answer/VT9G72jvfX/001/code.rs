// Answer 0

#[test]
fn test_next_valid_input() {
    struct TestRead {
        data: Vec<u8>,
        index: usize,
    }
    
    impl TestRead {
        fn new(data: Vec<u8>) -> Self {
            TestRead { data, index: 0 }
        }
    }

    impl Read<'static> for TestRead {
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

        fn peek(&mut self) -> Result<Option<u8>> { unimplemented!() }
        fn discard(&mut self) {}
        fn position(&self) { unimplemented!() }
        fn peek_position(&self) { unimplemented!() }
        fn byte_offset(&self) -> usize { self.index }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn begin_raw_buffering(&mut self) {}
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value> where V: Visitor<'static> { unimplemented!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = TestRead::new(vec![1, 2, 3]);
    
    assert_eq!(reader.next().unwrap(), Some(1));
    assert_eq!(reader.next().unwrap(), Some(2));
    assert_eq!(reader.next().unwrap(), Some(3));
    assert_eq!(reader.next().unwrap(), None);
}

#[test]
#[should_panic]
fn test_next_empty_input() {
    struct TestRead {
        data: Vec<u8>,
        index: usize,
    }
    
    impl TestRead {
        fn new(data: Vec<u8>) -> Self {
            TestRead { data, index: 0 }
        }
    }

    impl Read<'static> for TestRead {
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

        fn peek(&mut self) -> Result<Option<u8>> { unimplemented!() }
        fn discard(&mut self) {}
        fn position(&self) { unimplemented!() }
        fn peek_position(&self) { unimplemented!() }
        fn byte_offset(&self) -> usize { self.index }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn begin_raw_buffering(&mut self) {}
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value> where V: Visitor<'static> { unimplemented!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = TestRead::new(vec![]);
    let _ = reader.next().unwrap(); // This will not panic, expecting a None
}

#[test]
fn test_next_alternating_inputs() {
    struct TestRead {
        data: Vec<u8>,
        index: usize,
    }
    
    impl TestRead {
        fn new(data: Vec<u8>) -> Self {
            TestRead { data, index: 0 }
        }
    }

    impl Read<'static> for TestRead {
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

        fn peek(&mut self) -> Result<Option<u8>> { unimplemented!() }
        fn discard(&mut self) {}
        fn position(&self) { unimplemented!() }
        fn peek_position(&self) { unimplemented!() }
        fn byte_offset(&self) -> usize { self.index }
        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn begin_raw_buffering(&mut self) {}
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value> where V: Visitor<'static> { unimplemented!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = TestRead::new(vec![10, 20, 30, 40, 50]);

    assert_eq!(reader.next().unwrap(), Some(10));
    assert_eq!(reader.next().unwrap(), Some(20));
    assert_eq!(reader.next().unwrap(), Some(30));
    assert_eq!(reader.next().unwrap(), Some(40));
    assert_eq!(reader.next().unwrap(), Some(50));
    assert_eq!(reader.next().unwrap(), None);
}

