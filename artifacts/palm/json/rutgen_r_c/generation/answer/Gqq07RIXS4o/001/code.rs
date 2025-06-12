// Answer 0

#[test]
fn test_position_with_valid_slice() {
    struct MockSliceRead {
        slice: &'static [u8],
        index: usize,
    }

    impl MockSliceRead {
        fn position(&self) -> Position {
            Position { line: 1, column: self.index + 1 }
        }
    }

    let data: &[u8] = b"Hello, world!";
    let delegate = MockSliceRead { slice: data, index: 5 };
    let reader = StrRead { delegate, data: "Hello" };

    let result = reader.position();
    assert_eq!(result.line, 1);
    assert_eq!(result.column, 6);
}

#[test]
fn test_position_with_empty_slice() {
    struct MockSliceRead {
        slice: &'static [u8],
        index: usize,
    }

    impl MockSliceRead {
        fn position(&self) -> Position {
            Position { line: 1, column: self.index + 1 }
        }
    }

    let data: &[u8] = b"";
    let delegate = MockSliceRead { slice: data, index: 0 };
    let reader = StrRead { delegate, data: "" };

    let result = reader.position();
    assert_eq!(result.line, 1);
    assert_eq!(result.column, 1);
}

#[test]
fn test_position_with_large_index() {
    struct MockSliceRead {
        slice: &'static [u8],
        index: usize,
    }

    impl MockSliceRead {
        fn position(&self) -> Position {
            Position { line: 5, column: self.index }
        }
    }

    let data: &[u8] = b"abcdefghijklmnopqrstuvwx";
    let delegate = MockSliceRead { slice: data, index: 25 };
    let reader = StrRead { delegate, data: "a long string" };

    let result = reader.position();
    assert_eq!(result.line, 5);
    assert_eq!(result.column, 25);
}

