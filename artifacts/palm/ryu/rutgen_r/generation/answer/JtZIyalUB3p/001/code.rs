// Answer 0

#[test]
fn test_format_finite_with_valid_float() {
    struct Buffer {
        bytes: Vec<u8>,
    }

    impl Buffer {
        fn new(size: usize) -> Self {
            Buffer {
                bytes: vec![0; size],
            }
        }

        pub fn format_finite<F: Float>(&mut self, f: F) -> &str {
            unsafe {
                let n = f.write_to_ryu_buffer(self.bytes.as_mut_ptr() as *mut u8);
                debug_assert!(n <= self.bytes.len());
                let slice = std::slice::from_raw_parts(self.bytes.as_ptr() as *const u8, n);
                std::str::from_utf8_unchecked(slice)
            }
        }
    }

    struct FiniteFloat(f64);

    impl Float for FiniteFloat {
        fn write_to_ryu_buffer(&self, buffer: *mut u8) -> usize {
            let str_repr = format!("{}", self.0);
            let bytes = str_repr.as_bytes();
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer, bytes.len());
            bytes.len()
        }
    }

    let mut buffer = Buffer::new(32);
    let result = buffer.format_finite(FiniteFloat(3.14));
    assert_eq!(result, "3.14");
}

#[test]
fn test_format_finite_with_boundary_value() {
    struct Buffer {
        bytes: Vec<u8>,
    }

    impl Buffer {
        fn new(size: usize) -> Self {
            Buffer {
                bytes: vec![0; size],
            }
        }

        pub fn format_finite<F: Float>(&mut self, f: F) -> &str {
            unsafe {
                let n = f.write_to_ryu_buffer(self.bytes.as_mut_ptr() as *mut u8);
                debug_assert!(n <= self.bytes.len());
                let slice = std::slice::from_raw_parts(self.bytes.as_ptr() as *const u8, n);
                std::str::from_utf8_unchecked(slice)
            }
        }
    }

    struct BoundaryFloat(f64);

    impl Float for BoundaryFloat {
        fn write_to_ryu_buffer(&self, buffer: *mut u8) -> usize {
            let str_repr = format!("{}", self.0);
            let bytes = str_repr.as_bytes();
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer, bytes.len());
            bytes.len()
        }
    }

    let mut buffer = Buffer::new(32);
    let result = buffer.format_finite(BoundaryFloat(1.0e-10));
    assert_eq!(result, "1e-10");
}

#[test]
#[should_panic]
fn test_format_finite_with_non_finite_value() {
    struct Buffer {
        bytes: Vec<u8>,
    }

    impl Buffer {
        fn new(size: usize) -> Self {
            Buffer {
                bytes: vec![0; size],
            }
        }

        pub fn format_finite<F: Float>(&mut self, f: F) -> &str {
            unsafe {
                let n = f.write_to_ryu_buffer(self.bytes.as_mut_ptr() as *mut u8);
                debug_assert!(n <= self.bytes.len());
                let slice = std::slice::from_raw_parts(self.bytes.as_ptr() as *const u8, n);
                std::str::from_utf8_unchecked(slice)
            }
        }
    }

    struct NonFiniteFloat;

    impl Float for NonFiniteFloat {
        fn write_to_ryu_buffer(&self, _buffer: *mut u8) -> usize {
            panic!("Called with a non-finite value");
        }
    }

    let mut buffer = Buffer::new(32);
    buffer.format_finite(NonFiniteFloat);
}

