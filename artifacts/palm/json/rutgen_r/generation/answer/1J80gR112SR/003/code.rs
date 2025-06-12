// Answer 0

#[derive(Debug)]
struct MockVisitor {
    value: Result<i64, ()>,
}

impl<'any> de::Visitor<'any> for MockVisitor {
    type Value = i64;

    fn visit_i64(self, value: i64) -> Result<Self::Value> {
        Ok(value)
    }
}

struct MockDeserializer {
    input: Vec<u8>,
    index: usize,
}

impl MockDeserializer {
    fn new(input: &[u8]) -> Self {
        MockDeserializer {
            input: input.to_vec(),
            index: 0,
        }
    }

    fn parse_whitespace(&mut self) -> Result<Option<u8>> {
        // Simulate returning a whitespace followed by a number
        if self.index < self.input.len() {
            let byte = self.input[self.index];
            self.index += 1;
            return Ok(Some(byte));
        }
        Err(ErrorCode::EofWhileParsingValue)
    }

    fn parse_integer(&mut self, positive: bool) -> Result<i64> {
        let mut num = 0;
        while self.index < self.input.len() && self.input[self.index].is_ascii_digit() {
            num = num * 10 + (self.input[self.index] - b'0') as i64;
            self.index += 1;
        }
        Ok(num * if positive { 1 } else { -1 })
    }

    fn eat_char(&mut self) {
        if self.index < self.input.len() {
            self.index += 1;
        }
    }
    
    fn peek_error(&self, error: ErrorCode) -> Result<()> {
        Err(error)
    }
    
    fn peek_invalid_type<V>(&self, _visitor: &V) -> Result<()> {
        Err(ErrorCode::InvalidType)
    }
    
    fn fix_position(&self, _error: ()) -> Result<()> {
        Err(())
    }
}

#[test]
fn test_deserialize_number_positive() {
    let mut deserializer = MockDeserializer::new(b" 42");
    let visitor = MockVisitor { value: Ok(42) };
    let result = deserializer.deserialize_number(visitor);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_deserialize_number_negative() {
    let mut deserializer = MockDeserializer::new(b" -42");
    let visitor = MockVisitor { value: Ok(-42) };
    let result = deserializer.deserialize_number(visitor);
    assert_eq!(result, Ok(-42));
}

#[test]
fn test_deserialize_number_eof() {
    let mut deserializer = MockDeserializer::new(b" ");
    let visitor = MockVisitor { value: Ok(0) };
    let result = deserializer.deserialize_number(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_invalid_character() {
    let mut deserializer = MockDeserializer::new(b" x42");
    let visitor = MockVisitor { value: Ok(0) };
    let result = deserializer.deserialize_number(visitor);
    assert!(result.is_err());
}

