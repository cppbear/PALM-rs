// Answer 0

#[test]
fn test_end_object_value_initial() {
    use std::io;

    struct TestStruct {
        has_value: bool,
    }

    impl TestStruct {
        fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            self.has_value = true;
            Ok(())
        }
    }

    let mut test_struct = TestStruct { has_value: false };
    let mut writer = Vec::new();
    
    let result = test_struct.end_object_value(&mut writer);
    assert!(result.is_ok());
    assert!(test_struct.has_value);
}

#[test]
fn test_end_object_value_multiple_calls() {
    use std::io;

    struct TestStruct {
        has_value: bool,
    }

    impl TestStruct {
        fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            self.has_value = true;
            Ok(())
        }
    }

    let mut test_struct = TestStruct { has_value: false };
    let mut writer = Vec::new();
    
    let first_call = test_struct.end_object_value(&mut writer);
    assert!(first_call.is_ok());
    assert!(test_struct.has_value);

    // Reset state for second call
    test_struct.has_value = false;

    let second_call = test_struct.end_object_value(&mut writer);
    assert!(second_call.is_ok());
    assert!(test_struct.has_value);
}

