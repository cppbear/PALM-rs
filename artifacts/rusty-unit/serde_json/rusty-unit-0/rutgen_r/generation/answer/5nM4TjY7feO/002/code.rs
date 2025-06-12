// Answer 0

#[derive(Default)]
struct Peekable {
    data: Vec<u8>,
    index: usize,
}

impl Peekable {
    fn peek(&mut self) -> Result<u8, &'static str> {
        if self.index < self.data.len() {
            Ok(self.data[self.index])
        } else {
            Err("End of data")
        }
    }

    fn advance(&mut self) {
        if self.index < self.data.len() {
            self.index += 1;
        }
    }

    fn peek_or_null(&mut self) -> Result<u8, &'static str> {
        Ok(self.peek().unwrap_or(b'\x00'))
    }
}

#[test]
fn test_peek_or_null_with_valid_data() {
    let mut peekable = Peekable {
        data: vec![1, 2, 3],
        index: 0,
    };
    assert_eq!(peekable.peek_or_null().unwrap(), 1);
}

#[test]
fn test_peek_or_null_with_advanced_index() {
    let mut peekable = Peekable {
        data: vec![4, 5, 6],
        index: 1,
    };
    assert_eq!(peekable.peek_or_null().unwrap(), 5);
}

#[test]
fn test_peek_or_null_with_end_of_data() {
    let mut peekable = Peekable {
        data: vec![7, 8, 9],
        index: 3,
    };
    assert_eq!(peekable.peek_or_null().unwrap(), b'\x00');
}

#[test]
#[should_panic(expected = "End of data")]
fn test_peek_or_null_panics_on_invalid_peek() {
    let mut peekable = Peekable::default();
    let _ = peekable.peek_or_null();
}

