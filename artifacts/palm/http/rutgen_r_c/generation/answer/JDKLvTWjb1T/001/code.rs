// Answer 0

#[test]
fn test_as_str_valid_utf8() {
    struct TestStruct {
        ext: AllocatedExtension,
    }

    let valid_utf8_data = b"Hello, World!";
    let allocated_extension = AllocatedExtension::new(valid_utf8_data).expect("Failed to create AllocatedExtension");
    let test_instance = TestStruct { ext: allocated_extension };
    
    let result = test_instance.ext.as_str();
    assert_eq!(result, "Hello, World!");
}

#[test]
#[should_panic]
fn test_as_str_invalid_utf8() {
    struct TestStruct {
        ext: AllocatedExtension,
    }

    let invalid_utf8_data = [0xFF, 0xFE, 0xFD]; // Invalid UTF-8 sequence
    let _ = AllocatedExtension::new(&invalid_utf8_data).expect("Should not succeed creating AllocatedExtension with invalid UTF-8");
}

