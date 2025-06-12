// Answer 0

#[test]
fn test_new_byte_input() {
    struct ByteInput<'t> {
        text: &'t [u8],
        only_utf8: bool,
    }

    fn new<'t>(text: &'t [u8], only_utf8: bool) -> ByteInput<'t> {
        ByteInput {
            text: text,
            only_utf8: only_utf8,
        }
    }

    // Test case 1: Normal case with valid UTF-8 data
    let input = new(b"Hello, world!", true);
    assert_eq!(input.text, b"Hello, world!");
    assert!(input.only_utf8);

    // Test case 2: Empty input
    let input_empty = new(b"", false);
    assert_eq!(input_empty.text, b"");
    assert!(!input_empty.only_utf8);

    // Test case 3: Non-UTF-8 characters included, but only_utf8 is true
    let input_non_utf8 = new(&[0xFF, 0xFE, 0xFD], true);
    assert_eq!(input_non_utf8.text, &[0xFF, 0xFE, 0xFD]);
    assert!(input_non_utf8.only_utf8);

    // Test case 4: Non-UTF-8 characters included, and only_utf8 is false
    let input_non_utf8_false = new(&[0xFF, 0xFE, 0xFD], false);
    assert_eq!(input_non_utf8_false.text, &[0xFF, 0xFE, 0xFD]);
    assert!(!input_non_utf8_false.only_utf8);
}

