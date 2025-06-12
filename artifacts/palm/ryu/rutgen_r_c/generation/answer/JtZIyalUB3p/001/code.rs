// Answer 0

#[test]
fn test_format_finite_inner_buffer_size() {
    use core::mem::MaybeUninit;

    struct DummyFloat;

    impl Copy for DummyFloat {}

    impl Sealed for DummyFloat {
        fn is_nonfinite(self) -> bool {
            false
        }

        fn format_nonfinite(self) -> &'static str {
            ""
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let data = b"123456789012345678901234";
            let ptr = result as *mut u8;
            for (i, &byte) in data.iter().enumerate() {
                *ptr.add(i) = byte;
            }
            data.len()
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(DummyFloat);
    assert_eq!(result, "123456789012345678901234");
}

#[test]
fn test_format_finite_beyond_buffer_capacity() {
    struct OverflowFloat;

    impl Copy for OverflowFloat {}

    impl Sealed for OverflowFloat {
        fn is_nonfinite(self) -> bool {
            false
        }

        fn format_nonfinite(self) -> &'static str {
            ""
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            25 // Intentionally over the buffer size of 24
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(OverflowFloat);
    assert_eq!(result, ""); // We cannot determine the output since n > self.bytes.len(), it should be handled appropriately.
}

#[test]
#[should_panic]
fn test_format_finite_panic_on_invalid_utf8() {
    struct InvalidUtf8Float;

    impl Copy for InvalidUtf8Float {}

    impl Sealed for InvalidUtf8Float {
        fn is_nonfinite(self) -> bool {
            false
        }

        fn format_nonfinite(self) -> &'static str {
            ""
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let invalid_data = [0xff; 24]; // Invalid UTF-8 byte
            let ptr = result as *mut u8;
            for (i, &byte) in invalid_data.iter().enumerate() {
                *ptr.add(i) = byte;
            }
            invalid_data.len()
        }
    }

    let mut buffer = Buffer::new();
    buffer.format_finite(InvalidUtf8Float); // Expect this to panic due to invalid UTF-8
}

