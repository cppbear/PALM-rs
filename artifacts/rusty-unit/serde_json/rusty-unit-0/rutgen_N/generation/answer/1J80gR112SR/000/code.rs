// Answer 0

#[derive(Default)]
struct DummyVisitor {
    value: Option<i64>,
}

impl<'any> de::Visitor<'any> for DummyVisitor {
    type Value = i64;

    fn visit_i64(self, value: i64) -> Result<Self::Value> {
        self.value = Some(value);
        Ok(value)
    }
    
    fn visit_f64(self, value: f64) -> Result<Self::Value> {
        self.value = Some(value as i64);
        Ok(value as i64)
    }
}

struct DummyDeserializer {
    input: &'static [u8],
    position: usize,
}

impl DummyDeserializer {
    fn new(input: &'static [u8]) -> Self {
        Self { input, position: 0 }
    }

    fn parse_whitespace(&mut self) -> Option<u8> {
        while self.position < self.input.len() {
            let byte = self.input[self.position];
            if byte.is_ascii_whitespace() {
                self.position += 1;
            } else {
                return Some(byte);
            }
        }
        None
    }

    fn eat_char(&mut self) {
        self.position += 1;
    }

    fn parse_integer(&mut self, positive: bool) -> Result<i64> {
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
            self.position += 1;
        }
        let number_str = &self.input[start..self.position];
        let num = std::str::from_utf8(number_str).unwrap().parse::<i64>().map_err(|_| ErrorCode::ParseIntError)?;
        Ok(if positive { num } else { -num })
    }

    fn peek_error(&self, code: ErrorCode) -> Error {
        Error { message: format!("{:?}", code) }
    }

    fn peek_invalid_type(&self, visitor: &impl de::Visitor<'_>) -> Error {
        Error { message: format!("Invalid type for visitor: {:?}", visitor) }
    }

    fn fix_position(&self, err: Error) -> Error {
        Error { message: format!("Fixed position error: {}", err.message) }
    }
}

#[derive(Debug)]
struct Error {
    message: String,
}

#[derive(Debug)]
enum ErrorCode {
    EofWhileParsingValue,
    ParseIntError,
}

#[test]
fn test_deserialize_positive_integer() {
    let mut deserializer = DummyDeserializer::new(b"42");
    let visitor = DummyVisitor::default();
    let result = deserializer.deserialize_number(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_deserialize_negative_integer() {
    let mut deserializer = DummyDeserializer::new(b"-42");
    let visitor = DummyVisitor::default();
    let result = deserializer.deserialize_number(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), -42);
}

#[test]
fn test_deserialize_zero() {
    let mut deserializer = DummyDeserializer::new(b"0");
    let visitor = DummyVisitor::default();
    let result = deserializer.deserialize_number(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_deserialize_invalid_character() {
    let mut deserializer = DummyDeserializer::new(b"a");
    let visitor = DummyVisitor::default();
    let result = deserializer.deserialize_number(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_empty_input() {
    let mut deserializer = DummyDeserializer::new(b"");
    let visitor = DummyVisitor::default();
    let result = deserializer.deserialize_number(visitor);
    assert!(result.is_err());
}

