// Answer 0

#[test]
fn test_unexpected_bool_true() {
    let content = Content::Bool(true);
    let _result = content.unexpected();
}

#[test]
fn test_unexpected_bool_false() {
    let content = Content::Bool(false);
    let _result = content.unexpected();
}

