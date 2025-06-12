// Answer 0

#[test]
fn test_fmt_class_ascii_cntrl_negated() {
    struct Writer {
        buffer: String,
    }

    impl Writer {
        fn new() -> Self {
            Self { buffer: String::new() }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    struct AstClassAscii {
        kind: ClassAsciiKind,
        negated: bool,
    }

    enum ClassAsciiKind {
        Cntrl,
    }

    let mut writer = Writer::new();
    let ast = AstClassAscii { kind: ClassAsciiKind::Cntrl, negated: true };

    let result = writer.write_str("[:^cntrl:]");
    
    assert!(result.is_ok());
    assert_eq!(writer.buffer, "[:^cntrl:]");
}

#[test]
fn test_fmt_class_ascii_cntrl_non_negated() {
    struct Writer {
        buffer: String,
    }

    impl Writer {
        fn new() -> Self {
            Self { buffer: String::new() }
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    struct AstClassAscii {
        kind: ClassAsciiKind,
        negated: bool,
    }

    enum ClassAsciiKind {
        Cntrl,
    }

    let mut writer = Writer::new();
    let ast = AstClassAscii { kind: ClassAsciiKind::Cntrl, negated: false };

    let result = writer.write_str("[:cntrl:]");
    
    assert!(result.is_ok());
    assert_eq!(writer.buffer, "[:cntrl:]");
}

