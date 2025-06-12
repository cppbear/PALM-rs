// Answer 0

#[test]
fn test_invalid_type_string() {
    let unexp = de::Unexpected::Str("");
    let exp: &dyn de::Expected = &ExpectedString;
    invalid_type(unexp, exp);
}

#[test]
fn test_invalid_type_map() {
    let unexp = de::Unexpected::Map;
    let exp: &dyn de::Expected = &ExpectedMap;
    invalid_type(unexp, exp);
}

#[test]
fn test_invalid_type_seq() {
    let unexp = de::Unexpected::Seq;
    let exp: &dyn de::Expected = &ExpectedSeq;
    invalid_type(unexp, exp);
}

#[test]
fn test_invalid_type_borrowed_string() {
    let unexp = de::Unexpected::Borrowed("test");
    let exp: &dyn de::Expected = &ExpectedBorrowedString;
    invalid_type(unexp, exp);
}

#[test]
fn test_invalid_type_unit() {
    let unexp = de::Unexpected::Unit;
    let exp: &dyn de::Expected = &ExpectedUnit;
    invalid_type(unexp, exp);
}

#[test]
fn test_invalid_type_bool() {
    let unexp = de::Unexpected::Bool(true);
    let exp: &dyn de::Expected = &ExpectedBool;
    invalid_type(unexp, exp);
}

#[test]
fn test_invalid_type_number() {
    let unexp = de::Unexpected::Number(3.14);
    let exp: &dyn de::Expected = &ExpectedNumber;
    invalid_type(unexp, exp);
}

#[test]
fn test_invalid_type_other() {
    let unexp = de::Unexpected::Other;
    let exp: &dyn de::Expected = &ExpectedOther;
    invalid_type(unexp, exp);
}

