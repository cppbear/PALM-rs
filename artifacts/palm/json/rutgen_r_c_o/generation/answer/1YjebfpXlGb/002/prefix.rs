// Answer 0

#[test]
fn test_as_f32_neg_int_i8() {
    let number = Number::from(-1i8);
    number.as_f32();
}

#[test]
fn test_as_f32_neg_int_i16() {
    let number = Number::from(-32768i16);
    number.as_f32();
}

#[test]
fn test_as_f32_neg_int_i32() {
    let number = Number::from(-2147483648i32);
    number.as_f32();
}

#[test]
fn test_as_f32_neg_int_i64() {
    let number = Number::from(-9223372036854775807i64);
    number.as_f32();
}

#[test]
fn test_as_f32_neg_int_isize() {
    let number = Number::from(-1isize);
    number.as_f32();
}

