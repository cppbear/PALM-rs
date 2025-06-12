// Answer 0

#[test]
fn test_decode_engine_valid_input() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn decode<T: AsRef<[u8]>>(&self, input: T) -> Result<Vec<u8>, DecodeError> {
            let data = input.as_ref();
            if data == b"SGVsbG8gV29ybGQ=" {
                Ok(b"Hello World".to_vec())
            } else {
                Err(DecodeError::InvalidByte(0, 0))
            }
        }
    }

    let engine = MockEngine;
    let result = decode_engine("SGVsbG8gV29ybGQ=", &engine);
    assert_eq!(result, Ok(b"Hello World".to_vec()));
}

#[test]
fn test_decode_engine_invalid_byte() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn decode<T: AsRef<[u8]>>(&self, _input: T) -> Result<Vec<u8>, DecodeError> {
            Err(DecodeError::InvalidByte(0, 0xFF))
        }
    }

    let engine = MockEngine;
    let result = decode_engine("!!invalid!!", &engine);
    assert_eq!(result, Err(DecodeError::InvalidByte(0, 0xFF)));
}

#[test]
fn test_decode_engine_invalid_length() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn decode<T: AsRef<[u8]>>(&self, _input: T) -> Result<Vec<u8>, DecodeError> {
            Err(DecodeError::InvalidLength(1))
        }
    }

    let engine = MockEngine;
    let result = decode_engine("=", &engine);
    assert_eq!(result, Err(DecodeError::InvalidLength(1)));
}

#[test]
fn test_decode_engine_invalid_last_symbol() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn decode<T: AsRef<[u8]>>(&self, _input: T) -> Result<Vec<u8>, DecodeError> {
            Err(DecodeError::InvalidLastSymbol(2, b'X'))
        }
    }

    let engine = MockEngine;
    let result = decode_engine("YXZ=", &engine);
    assert_eq!(result, Err(DecodeError::InvalidLastSymbol(2, b'X')));
}

#[test]
fn test_decode_engine_invalid_padding() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn decode<T: AsRef<[u8]>>(&self, _input: T) -> Result<Vec<u8>, DecodeError> {
            Err(DecodeError::InvalidPadding)
        }
    }

    let engine = MockEngine;
    let result = decode_engine("U29tZSBpbnZhbGlkIHZhbGlk", &engine);
    assert_eq!(result, Err(DecodeError::InvalidPadding));
}

