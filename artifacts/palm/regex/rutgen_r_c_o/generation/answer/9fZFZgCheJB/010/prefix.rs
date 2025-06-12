// Answer 0

#[test]
fn test_is_capture_char_underscore_first() {
    let c = '_';
    let first = true;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_zero_not_first() {
    let c = '0';
    let first = false;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_a_not_first() {
    let c = 'a';
    let first = false;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_A_not_first() {
    let c = 'A';
    let first = false;
    is_capture_char(c, first);
}

