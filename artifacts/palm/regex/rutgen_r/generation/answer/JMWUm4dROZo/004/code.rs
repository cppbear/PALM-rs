// Answer 0

#[test]
fn test_fmt_set_flags_success() {
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

    struct TestAst {
        flags: String,
    }

    let mut writer = TestWriter::new();
    let ast = TestAst {
        flags: String::from("i"),
    };

    let result = fmt_set_flags(&mut writer, &ast);
    
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "(?i)");
}

#[test]
#[should_panic]
fn test_fmt_set_flags_write_str_panic() {
    struct PanicWriter;

    impl std::fmt::Write for PanicWriter {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            panic!("This writer panics!");
        }
    }

    struct TestAst {
        flags: String,
    }

    let mut writer = PanicWriter;
    let ast = TestAst {
        flags: String::from("m"),
    };

    let _ = fmt_set_flags(&mut writer, &ast);
}

#[test]
#[should_panic]
fn test_fmt_set_flags_fmt_flags_panic() {
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

    struct PanicAst {
        flags: String,
    }

    let mut writer = TestWriter::new();
    
    // This test will trigger a panic when fmt_flags is invoked
    let ast = PanicAst {
        flags: String::from("m"),
    };
    
    let _ = fmt_set_flags(&mut writer, &ast);
}

#[test]
fn test_fmt_set_flags_empty_flags() {
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

    struct TestAst {
        flags: String,
    }

    let mut writer = TestWriter::new();
    let ast = TestAst {
        flags: String::new(),
    };

    let result = fmt_set_flags(&mut writer, &ast);
    
    assert_eq!(result, Ok(()));
    assert_eq!(writer.output, "()"); 
}

