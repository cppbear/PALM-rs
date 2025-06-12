// Answer 0

#[test]
fn test_format_finite_f32() {
    struct Float32(f32);
    impl Copy for Float32 {}
    impl Sealed for Float32 {
        fn is_nonfinite(self) -> bool { !self.0.is_finite() }
        fn format_nonfinite(self) -> &'static str { NAN }
        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let str_rep = self.0.to_string();
            let str_bytes = str_rep.as_bytes();
            core::ptr::copy_nonoverlapping(str_bytes.as_ptr(), result, str_bytes.len());
            str_bytes.len()
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(Float32(3.14));
    assert_eq!(result, "3.14");
}

#[test]
fn test_format_finite_f64() {
    struct Float64(f64);
    impl Copy for Float64 {}
    impl Sealed for Float64 {
        fn is_nonfinite(self) -> bool { !self.0.is_finite() }
        fn format_nonfinite(self) -> &'static str { NAN }
        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let str_rep = self.0.to_string();
            let str_bytes = str_rep.as_bytes();
            core::ptr::copy_nonoverlapping(str_bytes.as_ptr(), result, str_bytes.len());
            str_bytes.len()
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(Float64(2.718));
    assert_eq!(result, "2.718");
}

#[test]
fn test_format_finite_f32_negative() {
    struct Float32(f32);
    impl Copy for Float32 {}
    impl Sealed for Float32 {
        fn is_nonfinite(self) -> bool { !self.0.is_finite() }
        fn format_nonfinite(self) -> &'static str { NAN }
        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let str_rep = self.0.to_string();
            let str_bytes = str_rep.as_bytes();
            core::ptr::copy_nonoverlapping(str_bytes.as_ptr(), result, str_bytes.len());
            str_bytes.len()
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(Float32(-1.0));
    assert_eq!(result, "-1");
}

#[test]
fn test_format_finite_f64_large() {
    struct Float64(f64);
    impl Copy for Float64 {}
    impl Sealed for Float64 {
        fn is_nonfinite(self) -> bool { !self.0.is_finite() }
        fn format_nonfinite(self) -> &'static str { NAN }
        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            let str_rep = self.0.to_string();
            let str_bytes = str_rep.as_bytes();
            core::ptr::copy_nonoverlapping(str_bytes.as_ptr(), result, str_bytes.len());
            str_bytes.len()
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format_finite(Float64(1234567890.123456));
    assert_eq!(result, "1234567890.123456");
}

