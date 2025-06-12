// Answer 0

#[test]
fn test_visit_str_valid_integer() {
    let visitor = FromStrVisitor::<i32> { expecting: "an integer", ty: PhantomData };
    let _ = visitor.visit_str("123");
}

#[test]
fn test_visit_str_valid_float() {
    let visitor = FromStrVisitor::<f32> { expecting: "a float", ty: PhantomData };
    let _ = visitor.visit_str("123.45");
}

#[test]
fn test_visit_str_empty_string() {
    let visitor = FromStrVisitor::<i32> { expecting: "an integer", ty: PhantomData };
    let _ = visitor.visit_str("");
}

#[test]
fn test_visit_str_whitespace_string() {
    let visitor = FromStrVisitor::<i32> { expecting: "an integer", ty: PhantomData };
    let _ = visitor.visit_str("   ");
}

#[test]
fn test_visit_str_invalid_integer() {
    let visitor = FromStrVisitor::<i32> { expecting: "an integer", ty: PhantomData };
    let _ = visitor.visit_str("abc");
}

#[test]
fn test_visit_str_invalid_float() {
    let visitor = FromStrVisitor::<f32> { expecting: "a float", ty: PhantomData };
    let _ = visitor.visit_str("12.34.56");
}

#[test]
fn test_visit_str_negative_integer() {
    let visitor = FromStrVisitor::<i32> { expecting: "an integer", ty: PhantomData };
    let _ = visitor.visit_str("-456");
}

#[test]
fn test_visit_str_large_integer() {
    let visitor = FromStrVisitor::<i64> { expecting: "an integer", ty: PhantomData };
    let _ = visitor.visit_str("9223372036854775807");
}

#[test]
fn test_visit_str_small_integer() {
    let visitor = FromStrVisitor::<i64> { expecting: "an integer", ty: PhantomData };
    let _ = visitor.visit_str("-9223372036854775808");
}

#[test]
fn test_visit_str_edge_case_float() {
    let visitor = FromStrVisitor::<f64> { expecting: "a float", ty: PhantomData };
    let _ = visitor.visit_str("1.7976931348623157e+308");
}

#[test]
fn test_visit_str_special_float() {
    let visitor = FromStrVisitor::<f32> { expecting: "a float", ty: PhantomData };
    let _ = visitor.visit_str("NaN");
}

#[test]
fn test_visit_str_special_float_infinity() {
    let visitor = FromStrVisitor::<f32> { expecting: "a float", ty: PhantomData };
    let _ = visitor.visit_str("Infinity");
}

