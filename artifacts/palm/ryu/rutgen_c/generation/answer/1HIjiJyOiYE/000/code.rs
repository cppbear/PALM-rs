// Answer 0

#[derive(Copy, Clone)]
struct TestFloatNaN;

#[derive(Copy, Clone)]
struct TestFloatInfinity;

#[derive(Copy, Clone)]
struct TestFloatFinite(f64);

impl Sealed for TestFloatNaN {
    fn is_nonfinite(self) -> bool {
        true
    }
    fn format_nonfinite(self) -> &'static str {
        NAN
    }
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
        // Simulate writing NaN
        let nan_str = b"NaN";
        core::ptr::copy_nonoverlapping(nan_str.as_ptr(), result, nan_str.len());
        nan_str.len()
    }
}

impl Sealed for TestFloatInfinity {
    fn is_nonfinite(self) -> bool {
        true
    }
    fn format_nonfinite(self) -> &'static str {
        INFINITY
    }
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
        // Simulate writing positive infinity
        let inf_str = b"inf";
        core::ptr::copy_nonoverlapping(inf_str.as_ptr(), result, inf_str.len());
        inf_str.len()
    }
}

impl Sealed for TestFloatFinite {
    fn is_nonfinite(self) -> bool {
        false
    }
    fn format_nonfinite(self) -> &'static str {
        unreachable!()
    }
    unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
        // Simulate writing finite float
        let finite_str = format!("{}", self.0);
        let finite_bytes = finite_str.as_bytes();
        core::ptr::copy_nonoverlapping(finite_bytes.as_ptr(), result, finite_bytes.len());
        finite_bytes.len()
    }
}

#[test]
fn test_format_nan() {
    let mut buffer = Buffer::new();
    let result = buffer.format(TestFloatNaN);
    assert_eq!(result, "NaN");
}

#[test]
fn test_format_positive_infinity() {
    let mut buffer = Buffer::new();
    let result = buffer.format(TestFloatInfinity);
    assert_eq!(result, "inf");
}

#[test]
fn test_format_finite_float() {
    let mut buffer = Buffer::new();
    let result = buffer.format(TestFloatFinite(42.0));
    assert_eq!(result, "42");
}

