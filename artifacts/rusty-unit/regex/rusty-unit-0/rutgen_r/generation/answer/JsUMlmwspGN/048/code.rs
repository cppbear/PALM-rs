// Answer 0

#[test]
fn test_utf8_decode_invalid_sequence() {
    struct InvalidSequence;

    impl InvalidSequence {
        fn create() -> Vec<u8> {
            vec![0b11110_0000, 0b10000010, 0b10111000, 0b11111111]
        }
    }

    let input = InvalidSequence::create();
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_utf8_decode_truncated_sequence() {
    struct TruncatedSequence;

    impl TruncatedSequence {
        fn create() -> Vec<u8> {
            vec![0b11110_0000, 0b10000010, 0b10111000]
        }
    }

    let input = TruncatedSequence::create();
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

#[test]
fn test_utf8_decode_surrogate_codepoint() {
    struct SurrogateCodepoint;

    impl SurrogateCodepoint {
        fn create() -> Vec<u8> {
            vec![0b11110_0000, 0b10000010, 0b10011111, 0b10111111]
        }
    }

    let input = SurrogateCodepoint::create();
    let result = decode_utf8(&input);
    assert_eq!(result, None);
}

