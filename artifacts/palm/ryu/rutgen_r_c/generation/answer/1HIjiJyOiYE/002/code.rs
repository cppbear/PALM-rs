// Answer 0

#[test]
fn test_format_finite_float() {
    struct TestFloat(f64);

    impl Copy for TestFloat {}
    
    impl Sealed for TestFloat {
        fn is_nonfinite(self) -> bool {
            self.0.is_nan() || self.0.is_infinite()
        }

        fn format_nonfinite(self) -> &'static str {
            NAN // Not used here since we are testing finite
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let formatted = format!("{}", self.0);
            let bytes = formatted.as_bytes();
            let len = bytes.len();
            ptr::copy_nonoverlapping(bytes.as_ptr(), result, len);
            len
        }
    }

    let mut buffer = Buffer::new();
    let float_value = TestFloat(3.14);
    
    let result = buffer.format(float_value);
    assert_eq!(result, "3.14");
}

#[test]
fn test_format_finite_negative_float() {
    struct TestFloat(f64);

    impl Copy for TestFloat {}

    impl Sealed for TestFloat {
        fn is_nonfinite(self) -> bool {
            self.0.is_nan() || self.0.is_infinite()
        }

        fn format_nonfinite(self) -> &'static str {
            NAN // Not used here since we are testing finite
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let formatted = format!("{}", self.0);
            let bytes = formatted.as_bytes();
            let len = bytes.len();
            ptr::copy_nonoverlapping(bytes.as_ptr(), result, len);
            len
        }
    }

    let mut buffer = Buffer::new();
    let float_value = TestFloat(-2.71);
    
    let result = buffer.format(float_value);
    assert_eq!(result, "-2.71");
}

#[test]
fn test_format_finite_large_float() {
    struct TestFloat(f64);

    impl Copy for TestFloat {}

    impl Sealed for TestFloat {
        fn is_nonfinite(self) -> bool {
            self.0.is_nan() || self.0.is_infinite()
        }

        fn format_nonfinite(self) -> &'static str {
            NAN // Not used here since we are testing finite
        }

        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let formatted = format!("{}", self.0);
            let bytes = formatted.as_bytes();
            let len = bytes.len();
            ptr::copy_nonoverlapping(bytes.as_ptr(), result, len);
            len
        }
    }

    let mut buffer = Buffer::new();
    let float_value = TestFloat(1e+10);
    
    let result = buffer.format(float_value);
    assert_eq!(result, "10000000000");
}

