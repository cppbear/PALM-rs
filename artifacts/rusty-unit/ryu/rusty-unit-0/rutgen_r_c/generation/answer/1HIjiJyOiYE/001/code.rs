// Answer 0

#[test]
fn test_format_nan() {
    struct NaNStruct;
    impl Copy for NaNStruct {}
    
    impl Sealed for NaNStruct {
        fn is_nonfinite(self) -> bool {
            true
        }
        
        fn format_nonfinite(self) -> &'static str {
            "NaN"
        }
        
        unsafe fn write_to_ryu_buffer(self, _: *mut u8) -> usize {
            0 // No need to write anything, we are testing non-finite values.
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(NaNStruct);
    assert_eq!(result, "NaN");
}

#[test]
fn test_format_infinity() {
    struct InfinityStruct;
    impl Copy for InfinityStruct {}
    
    impl Sealed for InfinityStruct {
        fn is_nonfinite(self) -> bool {
            true
        }
        
        fn format_nonfinite(self) -> &'static str {
            "inf"
        }
        
        unsafe fn write_to_ryu_buffer(self, _: *mut u8) -> usize {
            0 // No need to write anything, we are testing non-finite values.
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(InfinityStruct);
    assert_eq!(result, "inf");
}

#[test]
fn test_format_negative_infinity() {
    struct NegativeInfinityStruct;
    impl Copy for NegativeInfinityStruct {}
    
    impl Sealed for NegativeInfinityStruct {
        fn is_nonfinite(self) -> bool {
            true
        }
        
        fn format_nonfinite(self) -> &'static str {
            "-inf"
        }
        
        unsafe fn write_to_ryu_buffer(self, _: *mut u8) -> usize {
            0 // No need to write anything, we are testing non-finite values.
        }
    }

    let mut buffer = Buffer::new();
    let result = buffer.format(NegativeInfinityStruct);
    assert_eq!(result, "-inf");
}

