// Answer 0

#[test]
fn test_char_is_none_with_max_value() {
    let c = Char(4294967295);
    c.is_none();
}

#[test]
fn test_char_is_none_with_zero_value() {
    let c = Char(0);
    c.is_none();
}

#[test]
fn test_char_is_none_with_middle_value() {
    let c = Char(2147483648);
    c.is_none();
}

#[test]
fn test_char_is_none_with_small_value() {
    let c = Char(1);
    c.is_none();
}

#[test]
fn test_char_is_none_with_large_value_near_max() {
    let c = Char(4294967294);
    c.is_none();
}

