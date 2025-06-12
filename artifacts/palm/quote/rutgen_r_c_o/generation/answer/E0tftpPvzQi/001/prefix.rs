// Answer 0

#[test]
fn test_bool_span() {
    let value: bool = true;
    value.span();
}

#[test]
fn test_str_span() {
    let value: &str = "test string";
    value.span();
}

#[test]
fn test_string_span() {
    let value: String = String::from("test string");
    value.span();
}

#[test]
fn test_char_span() {
    let value: char = 'a';
    value.span();
}

#[test]
fn test_u8_span() {
    for value in [0u8, 128, 255].iter() {
        value.span();
    }
}

#[test]
fn test_u16_span() {
    for value in [0u16, 32768, 65535].iter() {
        value.span();
    }
}

#[test]
fn test_u32_span() {
    for value in [0u32, 2147483648, 4294967295].iter() {
        value.span();
    }
}

#[test]
fn test_u64_span() {
    for value in [0u64, 9223372036854775808, 18446744073709551615].iter() {
        value.span();
    }
}

#[test]
fn test_u128_span() {
    for value in [0u128, 170141183460469231731687303715884105728, 340282366920938463463374607431768211455].iter() {
        value.span();
    }
}

#[test]
fn test_usize_span() {
    let max_usize = std::usize::MAX;
    for value in [0usize, max_usize].iter() {
        value.span();
    }
}

