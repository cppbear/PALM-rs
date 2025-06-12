// Answer 0

#[test]
fn test_push_wtf8_codepoint_below_0x80() {
    let mut scratch = Vec::new();
    push_wtf8_codepoint(0x00, &mut scratch);
    assert_eq!(scratch, vec![0x00]); // Testing lower bound

    scratch.clear();
    push_wtf8_codepoint(0x1F, &mut scratch);
    assert_eq!(scratch, vec![0x1F]); // Testing another value below 0x80

    scratch.clear();
    push_wtf8_codepoint(0x7F, &mut scratch);
    assert_eq!(scratch, vec![0x7F]); // Testing upper bound for n < 0x80
}

