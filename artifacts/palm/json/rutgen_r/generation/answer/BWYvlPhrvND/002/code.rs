// Answer 0

fn next_char_or_null_test() -> Result<(), Box<dyn std::error::Error>> {
    struct TestStruct {
        next_char_result: Result<u8, &'static str>,
    }

    impl TestStruct {
        fn next_char(&mut self) -> Result<u8, &'static str> {
            self.next_char_result.clone()
        }
        
        fn next_char_or_null(&mut self) -> Result<u8, &'static str> {
            Ok(self.next_char()?.unwrap_or(b'\x00'))
        }
    }

    // Test case where `next_char` returns Ok(value)
    {
        let mut test_struct = TestStruct { next_char_result: Ok(b'a') };
        assert_eq!(test_struct.next_char_or_null()?, Ok(b'a'));
    }

    // Test case where `next_char` returns Ok(None)
    {
        let mut test_struct = TestStruct { next_char_result: Ok(b'\x00') };
        assert_eq!(test_struct.next_char_or_null()?, Ok(b'\x00'));
    }

    // Test case where `next_char` returns an error
    {
        let mut test_struct = TestStruct { next_char_result: Err("error") };
        assert!(test_struct.next_char_or_null().is_err());
    }

    Ok(())
}

#[test]
fn run_next_char_or_null_test() {
    if let Err(e) = next_char_or_null_test() {
        panic!("Test failed: {}", e);
    }
}

