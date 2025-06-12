// Answer 0

#[test]
fn test_advance_zero_steps() {
    struct CursorWrapper {
        cursor: std::io::Cursor<Vec<u8>>,
    }

    impl CursorWrapper {
        fn new(data: Vec<u8>) -> Self {
            CursorWrapper {
                cursor: std::io::Cursor::new(data),
            }
        }
    }

    let mut buffer = CursorWrapper::new(vec![0u8; 10]);
    let initial_position = buffer.cursor.position();
    buffer.cursor.advance(0);
    assert_eq!(buffer.cursor.position(), initial_position);
}

#[test]
fn test_advance_max_boundary() {
    struct CursorWrapper {
        cursor: std::io::Cursor<Vec<u8>>,
    }

    impl CursorWrapper {
        fn new(data: Vec<u8>) -> Self {
            CursorWrapper {
                cursor: std::io::Cursor::new(data),
            }
        }
    }

    let mut buffer = CursorWrapper::new(vec![0u8; 10]);
    buffer.cursor.set_position(5); // Set position to 5
    let max_cnt = 5; // length of buffer - current position
    buffer.cursor.advance(max_cnt as usize);
    assert_eq!(buffer.cursor.position(), 10);
}

#[should_panic(expected = "advance out of bounds: the len is 10 but advancing by 11")]
#[test]
fn test_advance_exceeds_length() {
    struct CursorWrapper {
        cursor: std::io::Cursor<Vec<u8>>,
    }

    impl CursorWrapper {
        fn new(data: Vec<u8>) -> Self {
            CursorWrapper {
                cursor: std::io::Cursor::new(data),
            }
        }
    }

    let mut buffer = CursorWrapper::new(vec![0u8; 10]);
    buffer.cursor.set_position(0);
    let exceeding_count = 11; // exceeds available
    buffer.cursor.advance(exceeding_count);
}

