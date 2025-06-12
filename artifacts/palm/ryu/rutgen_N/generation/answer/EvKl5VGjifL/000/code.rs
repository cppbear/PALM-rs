// Answer 0

#[test]
fn test_write_to_ryu_buffer() {
    struct TestInput {
        value: f32,
    }
    
    impl TestInput {
        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            raw::format32(self.value, result)
        }
    }
    
    let mut buffer = [0u8; 32];
    let input = TestInput { value: 1234.5 };
    
    let result_size = unsafe { input.write_to_ryu_buffer(buffer.as_mut_ptr()) };
    
    assert!(result_size > 0);
    assert_eq!(buffer[0], b'1'); // Check first byte for expected output
    // Additional assertions can be added based on expected formatting outcome
}

#[test]
#[should_panic]
fn test_write_to_ryu_buffer_null_pointer() {
    struct TestInput {
        value: f32,
    }
    
    impl TestInput {
        unsafe fn write_to_ryu_buffer(self, result: *mut u8) -> usize {
            raw::format32(self.value, result)
        }
    }
    
    let input = TestInput { value: 1234.5 };
    
    unsafe {
        let _ = input.write_to_ryu_buffer(std::ptr::null_mut());
    }
}

