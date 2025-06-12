// Answer 0

#[test]
fn test_fmt_with_slash_prefix() {
    struct TestStruct {
        data: String,
    }

    let test_instance = TestStruct {
        data: String::from("/path/to/resource"),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", &test_instance);
    assert!(result.is_ok());
    assert_eq!(output, "/path/to/resource");
}

#[test]
fn test_fmt_with_asterisk_prefix() {
    struct TestStruct {
        data: String,
    }

    let test_instance = TestStruct {
        data: String::from("*path/to/resource"),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", &test_instance);
    assert!(result.is_ok());
    assert_eq!(output, "*path/to/resource");
}

#[test]
fn test_fmt_empty_data() {
    struct TestStruct {
        data: String,
    }

    let test_instance = TestStruct {
        data: String::new(),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", &test_instance);
    assert!(result.is_ok());
    assert_eq!(output, "/");
}

