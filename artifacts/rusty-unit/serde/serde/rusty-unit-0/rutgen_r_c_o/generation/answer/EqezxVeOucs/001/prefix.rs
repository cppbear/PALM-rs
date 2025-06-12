// Answer 0

#[test]
fn test_as_str_unit() {
    let content = Content::Unit;
    content.as_str();
}

#[test]
fn test_as_str_none() {
    let content = Content::None;
    content.as_str();
}

#[test]
fn test_as_str_bool() {
    let content = Content::Bool(false);
    content.as_str();
}

#[test]
fn test_as_str_u8() {
    let content = Content::U8(0);
    content.as_str();
}

#[test]
fn test_as_str_i8() {
    let content = Content::I8(-1);
    content.as_str();
}

#[test]
fn test_as_str_f32() {
    let content = Content::F32(0.0);
    content.as_str();
}

#[test]
fn test_as_str_f64() {
    let content = Content::F64(0.0);
    content.as_str();
}

#[test]
fn test_as_str_some_unit() {
    let content = Content::Some(Box::new(Content::Unit));
    content.as_str();
}

