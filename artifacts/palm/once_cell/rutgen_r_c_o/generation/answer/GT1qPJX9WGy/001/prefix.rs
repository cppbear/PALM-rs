// Answer 0

#[test]
fn test_with_value_integer_0() {
    let _cell = OnceCell::with_value(0);
}

#[test]
fn test_with_value_integer_50() {
    let _cell = OnceCell::with_value(50);
}

#[test]
fn test_with_value_integer_100() {
    let _cell = OnceCell::with_value(100);
}

#[test]
fn test_with_value_char_a() {
    let _cell = OnceCell::with_value('a');
}

#[test]
fn test_with_value_char_z() {
    let _cell = OnceCell::with_value('z');
}

#[test]
fn test_with_value_bool_true() {
    let _cell = OnceCell::with_value(true);
}

#[test]
fn test_with_value_bool_false() {
    let _cell = OnceCell::with_value(false);
}

