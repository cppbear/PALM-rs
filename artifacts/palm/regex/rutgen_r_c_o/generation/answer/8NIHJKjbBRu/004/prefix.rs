// Answer 0

#[test]
fn test_next_utf8_four_byte_sequence() {
    let text = vec![0b1111_0001, 0b1000_0000, 0b1000_0000, 0b1000_0000];
    let i = 0;
    next_utf8(&text, i);
}

#[test]
fn test_next_utf8_panic_after_last_byte() {
    let text = vec![0b1111_0001, 0b1000_0000, 0b1000_0000];
    let i = 3; // Out of bounds
    next_utf8(&text, i);
}

#[test]
fn test_next_utf8_fourth_byte_sequence() {
    let text = vec![0b1111_0001, 0b1000_0001, 0b1000_0010, 0b1111_1111];
    let i = 0;
    next_utf8(&text, i);
}

#[test]
fn test_next_utf8_last_byte_four() {
    let text = vec![0b1111_0010, 0b1000_0100, 0b1000_0000, 0b1111_1111];
    let i = 2;
    next_utf8(&text, i);
}

#[test]
fn test_next_utf8_edge_case_fourth_byte() {
    let text = vec![0b1111_1000, 0b0000_0000, 0b1000_0001, 0b1000_0010];
    let i = 1;
    next_utf8(&text, i);
}

