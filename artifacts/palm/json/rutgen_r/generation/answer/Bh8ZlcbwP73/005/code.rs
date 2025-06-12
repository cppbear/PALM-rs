// Answer 0

#[test]
fn test_parse_str_bytes_no_scratch_no_escape() {
    struct TestInput<'a> {
        slice: &'a [u8],
        index: usize,
    }

    impl<'a> TestInput<'a> {
        fn skip_to_escape(&mut self, _validate: bool) {
            // No-op since slice is well-formed; we don't need to skip here.
        }
    }

    let mut input = TestInput {
        slice: b"\"valid string\"",
        index: 0,
    };

    let mut scratch = Vec::new();
    let validate = true;

    let result = input.parse_str_bytes(&mut scratch, validate, |_, borrowed| {
        assert_eq!(borrowed, b"valid string");
        Ok(borrowed)
    });

    assert!(result.is_ok());
}

#[test]
fn test_parse_str_bytes_with_scratch() {
    struct TestInput<'a> {
        slice: &'a [u8],
        index: usize,
    }

    impl<'a> TestInput<'a> {
        fn skip_to_escape(&mut self, _validate: bool) {
            // No-op since slice is well-formed; we don't need to skip here.
        }
    }

    let mut input = TestInput {
        slice: b"\"valid string\"",
        index: 0,
    };

    let mut scratch = Vec::new();
    let validate = true;

    let result = input.parse_str_bytes(&mut scratch, validate, |_, borrowed| {
        assert_eq!(borrowed, b"valid string");
        Ok(borrowed)
    });

    assert!(result.is_ok());
}

#[test]
fn test_parse_str_bytes_with_control_character() {
    struct TestInput<'a> {
        slice: &'a [u8],
        index: usize,
    }

    impl<'a> TestInput<'a> {
        fn skip_to_escape(&mut self, _validate: bool) {
            // Skip to invalid character
            self.index = self.slice.len() - 1; // Move to the last character
        }
    }

    let mut input = TestInput {
        slice: b"\"invalid\0string\"",
        index: 0,
    };

    let mut scratch = Vec::new();
    let validate = true;

    let result = input.parse_str_bytes(&mut scratch, validate, |_, _| {
        unreachable!(); // We expect this to panic before reaching here.
    });

    assert!(result.is_err());
}

