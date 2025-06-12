// Answer 0

#[derive(Default)]
struct Reader {
    state: u8,
}

impl Reader {
    fn next(&mut self) -> Result<Option<u8>, &'static str> {
        if self.state > 10 {
            return Err("State exceeded limit");
        }
        self.state += 1;
        if self.state > 5 {
            return Ok(None);
        }
        Ok(Some(self.state))
    }
}

#[test]
fn test_next_valid_states() {
    let mut reader = Reader::default();
    for expected in 1..=5 {
        let result = reader.next();
        assert_eq!(result, Ok(Some(expected)));
    }
}

#[test]
fn test_next_none_return() {
    let mut reader = Reader::default();
    reader.state = 5;
    let result = reader.next();
    assert_eq!(result, Ok(None));
}

#[test]
fn test_next_exceed_limit() {
    let mut reader = Reader::default();
    reader.state = 10;
    let result = reader.next();
    assert_eq!(result, Err("State exceeded limit"));
}

#[test]
fn test_next_panic_condition() {
    let mut reader = Reader::default();
    reader.state = 11;
    let result = reader.next();
    assert_eq!(result, Err("State exceeded limit"));
}

