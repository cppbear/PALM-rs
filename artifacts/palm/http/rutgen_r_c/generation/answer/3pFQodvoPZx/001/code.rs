// Answer 0

#[test]
fn test_parse_non_empty_with_empty_slice() {
    let authority = Authority::empty();
    let result = authority.parse_non_empty(&[]);
    assert_eq!(result, Err(ErrorKind::Empty.into()));
}

