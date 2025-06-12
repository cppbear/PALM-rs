// Answer 0

#[test]
fn test_finish_simple() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, val: &str) -> fmt::Result {
            self.output.push_str(val);
            Ok(())
        }
    }

    let mut printer = Printer { _priv: () };
    let mut writer = TestWriter { output: String::new() };
    let visitor = Writer { printer: &mut printer, wtr: &mut writer };

    let result = visitor.finish();
}

#[test]
fn test_finish_with_empty_writer() {
    struct EmptyWriter;

    impl fmt::Write for EmptyWriter {
        fn write_str(&mut self, _val: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut printer = Printer { _priv: () };
    let mut writer = EmptyWriter;
    let visitor = Writer { printer: &mut printer, wtr: &mut writer };

    let result = visitor.finish();
}

#[test]
fn test_finish_with_large_writer() {
    struct LargeWriter {
        output: String,
    }

    impl fmt::Write for LargeWriter {
        fn write_str(&mut self, val: &str) -> fmt::Result {
            self.output.push_str(val);
            if self.output.len() > 10_000 {
                return Err(fmt::Error);
            }
            Ok(())
        }
    }

    let mut printer = Printer { _priv: () };
    let mut writer = LargeWriter { output: String::new() };
    let visitor = Writer { printer: &mut printer, wtr: &mut writer };

    let result = visitor.finish();
} 

#[test]
#[should_panic]
fn test_finish_with_panic_in_writer() {
    struct PanicWriter;

    impl fmt::Write for PanicWriter {
        fn write_str(&mut self, _val: &str) -> fmt::Result {
            panic!("Intentional panic");
        }
    }

    let mut printer = Printer { _priv: () };
    let mut writer = PanicWriter;
    let visitor = Writer { printer: &mut printer, wtr: &mut writer };

    let result = visitor.finish();
} 

#[test]
fn test_finish_with_mutable_context() {
    struct MutableWriter {
        output: String,
    }

    impl fmt::Write for MutableWriter {
        fn write_str(&mut self, val: &str) -> fmt::Result {
            self.output.push_str(val);
            Ok(())
        }
    }

    let mut printer = Printer { _priv: () };
    let mut writer = MutableWriter { output: String::new() };
    {
        let mut visitor = Writer { printer: &mut printer, wtr: &mut writer };
        let result = visitor.finish();
    }
}

