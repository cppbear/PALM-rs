// Answer 0

#[test]
fn test_parse_non_empty_empty_input() {
    let input: &[u8] = b"";
    let result = parse_non_empty(input);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e, ErrorKind::Empty.into());
    }
}

