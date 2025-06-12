// Answer 0

#[test]
fn test_visit_alternation_in() {
    struct MockWriter {
        output: String,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }
    }
    
    impl MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestStruct {
        wtr: MockWriter,
    }

    impl TestStruct {
        fn visit_alternation_in(&mut self) -> std::fmt::Result {
            self.wtr.write_str("|")
        }
    }

    let mut test_struct = TestStruct {
        wtr: MockWriter::new(),
    };

    let result = test_struct.visit_alternation_in();
    assert!(result.is_ok());
    assert_eq!(test_struct.wtr.output, "|");
}

