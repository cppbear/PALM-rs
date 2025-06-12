// Answer 0

#[test]
fn test_fmt_debug_list_empty() {
    struct TestStruct {
        iter: Vec<Bucket>,
    }

    struct Bucket {
        value: String,
    }

    impl Bucket {
        fn value_ref(&self) -> &String {
            &self.value
        }
    }

    impl TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.as_slice().iter().map(Bucket::value_ref);
            f.debug_list().entries(iter).finish()
        }
    }

    let data = TestStruct { iter: Vec::new() };
    let result = format!("{:?}", data);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_debug_list_with_values() {
    struct TestStruct {
        iter: Vec<Bucket>,
    }

    struct Bucket {
        value: String,
    }

    impl Bucket {
        fn value_ref(&self) -> &String {
            &self.value
        }
    }

    impl TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.as_slice().iter().map(Bucket::value_ref);
            f.debug_list().entries(iter).finish()
        }
    }

    let data = TestStruct {
        iter: vec![Bucket { value: "value1".to_string() }, Bucket { value: "value2".to_string() }],
    };
    let result = format!("{:?}", data);
    assert_eq!(result, "[\"value1\", \"value2\"]");
}

#[test]
fn test_fmt_debug_list_single_value() {
    struct TestStruct {
        iter: Vec<Bucket>,
    }

    struct Bucket {
        value: String,
    }

    impl Bucket {
        fn value_ref(&self) -> &String {
            &self.value
        }
    }

    impl TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter.as_slice().iter().map(Bucket::value_ref);
            f.debug_list().entries(iter).finish()
        }
    }

    let data = TestStruct {
        iter: vec![Bucket { value: "single_value".to_string() }],
    };
    let result = format!("{:?}", data);
    assert_eq!(result, "[\"single_value\"]");
}

