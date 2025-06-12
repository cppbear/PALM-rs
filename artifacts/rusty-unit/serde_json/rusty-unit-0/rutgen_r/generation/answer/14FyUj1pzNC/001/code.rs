// Answer 0


struct DummyReader;

impl DummyReader {
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<&'s [u8], &'static str> {
        if scratch.is_empty() {
            return Err("Scratch buffer is empty");
        }
        Ok(&scratch)
    }
}

#[test]
fn test_parse_str_raw_with_non_empty_buffer() {
    let mut reader = DummyReader;
    let mut scratch = vec![1, 2, 3, 4];
    let result = reader.parse_str_raw(&mut scratch);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), &[1, 2, 3, 4]);
}

#[test]
#[should_panic(expected = "Scratch buffer is empty")]
fn test_parse_str_raw_with_empty_buffer() {
    let mut reader = DummyReader;
    let mut scratch = Vec::new();
    let _result = reader.parse_str_raw(&mut scratch).unwrap();
}


