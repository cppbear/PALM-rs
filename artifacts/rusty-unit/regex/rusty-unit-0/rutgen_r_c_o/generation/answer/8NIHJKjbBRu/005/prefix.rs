// Answer 0

#[test]
fn test_next_utf8_empty_text() {
    let text: &[u8] = &[];
    let i = 0;
    next_utf8(text, i);
}

#[test]
fn test_next_utf8_single_index_out_of_bounds() {
    let text: &[u8] = &[0x61]; // 'a'
    let i = 1; // out of bounds
    next_utf8(text, i);
}

#[test]
fn test_next_utf8_two_elements_index_out_of_bounds() {
    let text: &[u8] = &[0x61, 0x62]; // 'a', 'b'
    let i = 2; // out of bounds
    next_utf8(text, i);
}

#[test]
fn test_next_utf8_boundary_case() {
    let text: &[u8] = &[0x61, 0x62, 0x63]; // 'a', 'b', 'c'
    let i = 3; // out of bounds
    next_utf8(text, i);
}

