// Answer 0

#[test]
fn test_position() {
    struct TestSliceRead<'a> {
        slice: &'a [u8],
        index: usize,
    }

    impl<'a> SliceRead<'a> {
        fn new(slice: &'a [u8]) -> Self {
            Self {
                slice,
                index: 0,
            }
        }
        
        fn position(&self) -> Position {
            Position { line: 1, column: self.index + 1 }
        }
    }

    let test_data: &[u8] = b"test";
    let slice_read = TestSliceRead::new(test_data);
    
    let str_read = StrRead {
        delegate: slice_read,
        #[cfg(feature = "raw_value")]
        data: "test string",
    };

    let pos = str_read.position();
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 1);
}

