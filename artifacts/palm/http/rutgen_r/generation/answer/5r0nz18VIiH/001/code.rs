// Answer 0

#[test]
fn test_fmt_with_valid_string() {
    struct TestStruct {
        value: &'static str,
    }

    impl TestStruct {
        fn as_str(&self) -> &str {
            self.value
        }
    }

    let test_instance = TestStruct { value: "valid string" };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", test_instance);
    assert!(result.is_ok());
    assert_eq!(buffer, "\"valid string\"");
}

#[test]
fn test_fmt_with_empty_string() {
    struct TestStruct {
        value: &'static str,
    }

    impl TestStruct {
        fn as_str(&self) -> &str {
            self.value
        }
    }

    let test_instance = TestStruct { value: "" };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", test_instance);
    assert!(result.is_ok());
    assert_eq!(buffer, "\"\"");
}

#[test]
fn test_fmt_with_special_characters() {
    struct TestStruct {
        value: &'static str,
    }

    impl TestStruct {
        fn as_str(&self) -> &str {
            self.value
        }
    }

    let test_instance = TestStruct { value: "special chars !@#$%^&*()" };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", test_instance);
    assert!(result.is_ok());
    assert_eq!(buffer, "\"special chars !@#$%^&*()\"");
}

#[should_panic]
fn test_fmt_with_null_string() {
    struct TestStruct {
        value: *const i8,
    }

    impl TestStruct {
        fn as_str(&self) -> &str {
            unsafe {
                std::ffi::CStr::from_ptr(self.value).to_str().unwrap()
            }
        }
    }

    let test_instance = TestStruct { value: std::ptr::null() };
    let _ = format!("{:?}", test_instance);
}

