// Answer 0

#[test]
fn test_from_name_graph() {
    use crate::ClassAsciiKind;

    let result = from_name("graph");
    assert_eq!(result, Some(ClassAsciiKind::Graph));
}

#[test]
fn test_from_name_invalid() {
    assert_eq!(from_name("alnum"), None);
    assert_eq!(from_name("alpha"), None);
    assert_eq!(from_name("ascii"), None);
    assert_eq!(from_name("blank"), None);
    assert_eq!(from_name("cntrl"), None);
    assert_eq!(from_name("digit"), None);
    assert_eq!(from_name("lower"), None);
    assert_eq!(from_name("print"), None);
    assert_eq!(from_name("punct"), None);
    assert_eq!(from_name("space"), None);
    assert_eq!(from_name("upper"), None);
    assert_eq!(from_name("word"), None);
    assert_eq!(from_name("xdigit"), None);
    assert_eq!(from_name("nonexistent"), None);
}

