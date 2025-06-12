// Answer 0

#[test]
fn test_fmt_cut() {
    struct TestStruct {
        v: String,
    }

    impl TestStruct {
        fn is_cut(&self) -> bool {
            true
        }
    }

    let test_instance = TestStruct {
        v: String::from("test_string"),
    };
    
    let mut output = String::new();
    let result = write!(&mut output, "Cut({})", escape_unicode(&test_instance.v));

    assert!(result.is_ok());
    assert_eq!(output, "Cut(\\u{74}\\u{65}\\u{73}\\u{74}\\u{5f}\\u{73}\\u{74}\\u{72}\\u{69}\\u{6e}\\u{67})");
}

#[test]
fn test_fmt_complete() {
    struct TestStruct {
        v: String,
    }

    impl TestStruct {
        fn is_cut(&self) -> bool {
            false
        }
    }

    let test_instance = TestStruct {
        v: String::from("test_string"),
    };
    
    let mut output = String::new();
    let result = write!(&mut output, "Complete({})", escape_unicode(&test_instance.v));

    assert!(result.is_ok());
    assert_eq!(output, "Complete(\\u{74}\\u{65}\\u{73}\\u{74}\\u{5f}\\u{73}\\u{74}\\u{72}\\u{69}\\u{6e}\\u{67})");
}

