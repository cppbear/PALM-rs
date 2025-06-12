// Answer 0

#[test]
fn test_from_name_valid_xdigit() {
    assert_eq!(regex_syntax::ClassAsciiKind::from_name("xdigit"), Some(regex_syntax::ClassAsciiKind::Xdigit));
}

#[test]
fn test_from_name_invalid_names() {
    assert_eq!(regex_syntax::ClassAsciiKind::from_name("alnum"), None);
    assert_eq!(regex_syntax::ClassAsciiKind::from_name("alpha"), None);
    assert_eq!(regex_syntax::ClassAsciiKind::from_name("ascii"), None);
    assert_eq!(regex_syntax::ClassAsciiKind::from_name("blank"), None);
    assert_eq!(regex_syntax::ClassAsciiKind::from_name("cntrl"), None);
    assert_eq!(regex_syntax::ClassAsciiKind::from_name("digit"), None);
    assert_eq!(regex_syntax::ClassAsciiKind::from_name("graph"), None);
    assert_eq!(regex_syntax::ClassAsciiKind::from_name("lower"), None);
    assert_eq!(regex_syntax::ClassAsciiKind::from_name("print"), None);
    assert_eq!(regex_syntax::ClassAsciiKind::from_name("punct"), None);
    assert_eq!(regex_syntax::ClassAsciiKind::from_name("space"), None);
    assert_eq!(regex_syntax::ClassAsciiKind::from_name("upper"), None);
    assert_eq!(regex_syntax::ClassAsciiKind::from_name("word"), None);
}

