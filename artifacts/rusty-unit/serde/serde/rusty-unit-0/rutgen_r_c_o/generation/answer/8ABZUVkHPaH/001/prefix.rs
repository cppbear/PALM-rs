// Answer 0

#[test]
fn test_visit_str_empty() {
    let mut s = String::new();
    let visitor = StringInPlaceVisitor(&mut s);
    let _ = visitor.visit_str("");
}

#[test]
fn test_visit_str_short() {
    let mut s = String::new();
    let visitor = StringInPlaceVisitor(&mut s);
    let _ = visitor.visit_str("short string");
}

#[test]
fn test_visit_str_long() {
    let mut s = String::new();
    let visitor = StringInPlaceVisitor(&mut s);
    let test_str = "long string that is within reasonable maximum length for testing purposes";
    let _ = visitor.visit_str(test_str);
}

#[test]
fn test_visit_str_special_chars() {
    let mut s = String::new();
    let visitor = StringInPlaceVisitor(&mut s);
    let _ = visitor.visit_str("!@#$%^&*()_+[]{}|;:',.<>?");
}

#[test]
fn test_visit_str_unicode() {
    let mut s = String::new();
    let visitor = StringInPlaceVisitor(&mut s);
    let _ = visitor.visit_str("Unicode: 你好, Привет, مرحبا");
}

#[test]
fn test_visit_str_consecutive_calls() {
    let mut s = String::new();
    let visitor = StringInPlaceVisitor(&mut s);
    let _ = visitor.visit_str("first string");
    let _ = visitor.visit_str("second string");
    let _ = visitor.visit_str("");
}

