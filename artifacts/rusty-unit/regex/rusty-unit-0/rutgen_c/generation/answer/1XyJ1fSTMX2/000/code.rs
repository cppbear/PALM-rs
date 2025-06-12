// Answer 0

#[test]
fn test_at_valid_index() {
    struct TestByteInput<'t> {
        text: &'t [u8],
    }

    impl<'t> ByteInput<'t> {
        fn get(&self, index: usize) -> Option<&u8> {
            self.text.get(index)
        }
    }

    let input = TestByteInput {
        text: b"Hello, world!",
    };

    let result = input.at(0);
    assert_eq!(result.pos, 0);
    assert_eq!(result.byte, Some(&b'H'));
    assert_eq!(result.len, 1);
}

#[test]
fn test_at_index_out_of_bounds() {
    struct TestByteInput<'t> {
        text: &'t [u8],
    }

    impl<'t> ByteInput<'t> {
        fn get(&self, index: usize) -> Option<&u8> {
            self.text.get(index)
        }
    }

    let input = TestByteInput {
        text: b"Hello, world!",
    };

    let result = input.at(20); // Index out of bounds
    assert_eq!(result.pos, 20);
    assert_eq!(result.byte, None);
    assert_eq!(result.len, 1);
}

#[test]
fn test_at_empty_input() {
    struct TestByteInput<'t> {
        text: &'t [u8],
    }

    impl<'t> ByteInput<'t> {
        fn get(&self, index: usize) -> Option<&u8> {
            self.text.get(index)
        }
    }

    let input = TestByteInput {
        text: &[],
    };

    let result = input.at(0); // Even though index is 0, the input is empty
    assert_eq!(result.pos, 0);
    assert_eq!(result.byte, None);
    assert_eq!(result.len, 1);
}

