// Answer 0

#[test]
fn test_format_nan() {
    struct TestFloat;
    
    impl Sealed for TestFloat {
        fn is_nonfinite(self) -> bool { true }
        fn format_nonfinite(self) -> &'static str { NAN }
        unsafe fn write_to_ryu_buffer(self, _: *mut u8) -> usize { 0 }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(TestFloat);
}

#[test]
fn test_format_infinity() {
    struct TestFloat;
    
    impl Sealed for TestFloat {
        fn is_nonfinite(self) -> bool { true }
        fn format_nonfinite(self) -> &'static str { INFINITY }
        unsafe fn write_to_ryu_buffer(self, _: *mut u8) -> usize { 0 }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(TestFloat);
}

#[test]
fn test_format_negative_infinity() {
    struct TestFloat;
    
    impl Sealed for TestFloat {
        fn is_nonfinite(self) -> bool { true }
        fn format_nonfinite(self) -> &'static str { NEG_INFINITY }
        unsafe fn write_to_ryu_buffer(self, _: *mut u8) -> usize { 0 }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(TestFloat);
}

