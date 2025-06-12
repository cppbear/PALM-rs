// Answer 0

#[test]
fn test_expecting_i8() {
    let mut formatter = std::fmt::Formatter::new();
    let visitor = RangeFromVisitor::<i8> { expecting: "an i8", phantom: std::marker::PhantomData };
    visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_i16() {
    let mut formatter = std::fmt::Formatter::new();
    let visitor = RangeFromVisitor::<i16> { expecting: "an i16", phantom: std::marker::PhantomData };
    visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_i32() {
    let mut formatter = std::fmt::Formatter::new();
    let visitor = RangeFromVisitor::<i32> { expecting: "an i32", phantom: std::marker::PhantomData };
    visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_i64() {
    let mut formatter = std::fmt::Formatter::new();
    let visitor = RangeFromVisitor::<i64> { expecting: "an i64", phantom: std::marker::PhantomData };
    visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_u8() {
    let mut formatter = std::fmt::Formatter::new();
    let visitor = RangeFromVisitor::<u8> { expecting: "an u8", phantom: std::marker::PhantomData };
    visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_u16() {
    let mut formatter = std::fmt::Formatter::new();
    let visitor = RangeFromVisitor::<u16> { expecting: "an u16", phantom: std::marker::PhantomData };
    visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_u32() {
    let mut formatter = std::fmt::Formatter::new();
    let visitor = RangeFromVisitor::<u32> { expecting: "an u32", phantom: std::marker::PhantomData };
    visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_u64() {
    let mut formatter = std::fmt::Formatter::new();
    let visitor = RangeFromVisitor::<u64> { expecting: "an u64", phantom: std::marker::PhantomData };
    visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_f32() {
    let mut formatter = std::fmt::Formatter::new();
    let visitor = RangeFromVisitor::<f32> { expecting: "a f32", phantom: std::marker::PhantomData };
    visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_f64() {
    let mut formatter = std::fmt::Formatter::new();
    let visitor = RangeFromVisitor::<f64> { expecting: "a f64", phantom: std::marker::PhantomData };
    visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_char() {
    let mut formatter = std::fmt::Formatter::new();
    let visitor = RangeFromVisitor::<char> { expecting: "a char", phantom: std::marker::PhantomData };
    visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_string() {
    let mut formatter = std::fmt::Formatter::new();
    let visitor = RangeFromVisitor::<String> { expecting: "a string", phantom: std::marker::PhantomData };
    visitor.expecting(&mut formatter);
}

