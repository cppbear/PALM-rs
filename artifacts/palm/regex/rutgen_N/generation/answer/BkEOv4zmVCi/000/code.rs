// Answer 0

#[test]
fn test_visit_alternation_in() {
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestStruct {
        wtr: TestWriter,
    }

    impl TestStruct {
        fn new() -> Self {
            TestStruct {
                wtr: TestWriter::new(),
            }
        }

        fn visit_alternation_in(&mut self) -> std::fmt::Result {
            self.wtr.write_str("|")
        }
    }

    let mut test_struct = TestStruct::new();
    let result = test_struct.visit_alternation_in();

    assert!(result.is_ok());
    assert_eq!(test_struct.wtr.output, "|");
}

