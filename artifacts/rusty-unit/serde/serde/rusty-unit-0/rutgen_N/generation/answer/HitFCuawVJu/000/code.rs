// Answer 0

#[test]
fn test_expecting() {
    struct TestExpecting {
        expecting: &'static str,
    }

    impl TestExpecting {
        fn new(expectation: &'static str) -> Self {
            TestExpecting { expecting: expectation }
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str(self.expecting)
        }
    }

    let test_instance = TestExpecting::new("test expectation");
    let mut output = String::new();
    let result = test_instance.expecting(&mut output).unwrap();

    assert_eq!(output, "test expectation");
    assert!(result.is_ok());
}

