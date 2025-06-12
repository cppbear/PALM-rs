// Answer 0

#[test]
fn test_fmt() {
    struct TestStruct {
        value: String,
    }

    impl TestStruct {
        fn s(&self) -> &String {
            &self.value
        }
    }

    let test_instance = TestStruct {
        value: String::from("Hello, world!"),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", test_instance);

    assert!(result.is_ok());
    assert_eq!(output, "Hello, world!");
}

