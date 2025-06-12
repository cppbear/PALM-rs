// Answer 0

#[derive(Debug)]
struct DummyDeserializer {
    input: Vec<u8>,
    index: usize,
}

impl DummyDeserializer {
    fn next_char_or_null(&mut self) -> Result<u8, ()> {
        if self.index < self.input.len() {
            let val = self.input[self.index];
            self.index += 1;
            Ok(val)
        } else {
            Ok(0) // Simulating null-like behavior
        }
    }

    fn peek_or_null(&mut self) -> Result<u8, ()> {
        if self.index < self.input.len() {
            Ok(self.input[self.index])
        } else {
            Ok(0) // Simulating null-like behavior
        }
    }

    fn eat_char(&mut self) {
        if self.index < self.input.len() {
            self.index += 1;
        }
    }

    fn error(&self, _code: ()) -> Result<(), ()> {
        Err(())
    }

    fn ignore_decimal(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn ignore_exponent(&mut self) -> Result<(), ()> {
        Ok(())
    }
}

fn tri<T>(result: Result<T, ()>) -> T {
    result.unwrap()
}

#[test]
fn test_ignore_integer_invalid_start_with_multiple_leading_zeros() {
    let mut deserializer = DummyDeserializer { input: b"00".to_vec(), index: 0 };
    let result = deserializer.ignore_integer();
    assert!(result.is_err());
}

#[test]
fn test_ignore_integer_invalid_start_with_non_digit() {
    let mut deserializer = DummyDeserializer { input: b"a1".to_vec(), index: 0 };
    let result = deserializer.ignore_integer();
    assert!(result.is_err());
}

#[test]
fn test_ignore_integer_valid_with_no_decimal_or_exponent() {
    let mut deserializer = DummyDeserializer { input: b"123".to_vec(), index: 0 };
    let result = deserializer.ignore_integer();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_integer_valid_with_decimal() {
    let mut deserializer = DummyDeserializer { input: b"123.456".to_vec(), index: 0 };
    let result = deserializer.ignore_integer();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_integer_valid_with_exponent() {
    let mut deserializer = DummyDeserializer { input: b"123e10".to_vec(), index: 0 };
    let result = deserializer.ignore_integer();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_integer_invalid_character_after_integer() {
    let mut deserializer = DummyDeserializer { input: b"123a".to_vec(), index: 0 };
    let result = deserializer.ignore_integer();
    assert!(result.is_err());
}

