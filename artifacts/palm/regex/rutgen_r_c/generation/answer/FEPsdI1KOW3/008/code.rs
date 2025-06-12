// Answer 0

#[test]
fn test_from_name_lower() {
    use regex_syntax::ClassAsciiKind;

    let result = ClassAsciiKind::from_name("lower");
    assert_eq!(result, Some(ClassAsciiKind::Lower));
}

#[test]
fn test_from_name_invalid() {
    use regex_syntax::ClassAsciiKind;

    let result_alnum = ClassAsciiKind::from_name("alnum");
    let result_alpha = ClassAsciiKind::from_name("alpha");
    let result_ascii = ClassAsciiKind::from_name("ascii");
    let result_blank = ClassAsciiKind::from_name("blank");
    let result_cntrl = ClassAsciiKind::from_name("cntrl");
    let result_digit = ClassAsciiKind::from_name("digit");
    let result_graph = ClassAsciiKind::from_name("graph");
    
    assert_eq!(result_alnum, None);
    assert_eq!(result_alpha, None);
    assert_eq!(result_ascii, None);
    assert_eq!(result_blank, None);
    assert_eq!(result_cntrl, None);
    assert_eq!(result_digit, None);
}

