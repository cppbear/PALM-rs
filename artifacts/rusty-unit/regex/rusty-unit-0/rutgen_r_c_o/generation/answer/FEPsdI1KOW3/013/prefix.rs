// Answer 0

#[test]
fn test_from_name_word() {
    let result = ClassAsciiKind::from_name("word");
}

#[test]
fn test_from_name_invalid() {
    let result1 = ClassAsciiKind::from_name("alnum");
    let result2 = ClassAsciiKind::from_name("alpha");
    let result3 = ClassAsciiKind::from_name("ascii");
    let result4 = ClassAsciiKind::from_name("blank");
    let result5 = ClassAsciiKind::from_name("cntrl");
    let result6 = ClassAsciiKind::from_name("digit");
    let result7 = ClassAsciiKind::from_name("graph");
    let result8 = ClassAsciiKind::from_name("lower");
    let result9 = ClassAsciiKind::from_name("print");
    let result10 = ClassAsciiKind::from_name("punct");
    let result11 = ClassAsciiKind::from_name("space");
    let result12 = ClassAsciiKind::from_name("upper");
    let result13 = ClassAsciiKind::from_name("xdigit");
    let result14 = ClassAsciiKind::from_name("some_invalid_name");
}

