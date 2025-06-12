// Answer 0

#[test]
fn test_lower_case() {
    let result = from_name("lower");
}

#[test]
fn test_invalid_names() {
    let result1 = from_name("alnum");
    let result2 = from_name("alpha");
    let result3 = from_name("ascii");
    let result4 = from_name("blank");
    let result5 = from_name("cntrl");
    let result6 = from_name("digit");
    let result7 = from_name("graph");
    let result8 = from_name("notarealname");
}

