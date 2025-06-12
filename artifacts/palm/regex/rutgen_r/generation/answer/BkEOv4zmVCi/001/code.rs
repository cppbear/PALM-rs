// Answer 0

struct TestWriter {
    output: String,
}

impl TestWriter {
    fn new() -> Self {
        Self { output: String::new() }
    }
}

impl std::fmt::Write for TestWriter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.output.push_str(s);
        Ok(())
    }
}

struct TestStruct<'a> {
    wtr: &'a mut TestWriter,
}

impl<'a> TestStruct<'a> {
    fn visit_alternation_in(&mut self) -> std::fmt::Result {
        self.wtr.write_str("|")
    }
}

#[test]
fn test_visit_alternation_in() {
    let mut writer = TestWriter::new();
    let mut test_struct = TestStruct { wtr: &mut writer };
    
    let result = test_struct.visit_alternation_in();
    assert!(result.is_ok());
    assert_eq!(writer.output, "|");
}

#[test]
#[should_panic]
fn test_visit_alternation_in_should_panic() {
    // Intentionally not handling the write to trigger a panic
    struct PanicWriter;

    impl std::fmt::Write for PanicWriter {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            panic!("Intentional panic");
        }
    }

    let mut panic_writer = PanicWriter;
    let mut test_struct = TestStruct { wtr: &mut panic_writer };
    
    let _ = test_struct.visit_alternation_in();
}

