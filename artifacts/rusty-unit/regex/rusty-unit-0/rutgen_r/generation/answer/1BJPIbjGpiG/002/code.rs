// Answer 0

#[test]
fn test_fmt_literal_special_carriage_return() {
    struct MockWriter {
        output: String,
    }
    
    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::Literal {
        kind: ast::SpecialLiteralKind::CarriageReturn.into(),
        c: '\r' as u32,
    };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\r");
}

#[test]
fn test_fmt_literal_special_vertical_tab() {
    struct MockWriter {
        output: String,
    }
    
    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::Literal {
        kind: ast::SpecialLiteralKind::VerticalTab.into(),
        c: '\x0B' as u32,
    };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\v");
}

#[test]
fn test_fmt_literal_special_space() {
    struct MockWriter {
        output: String,
    }
    
    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::Literal {
        kind: ast::SpecialLiteralKind::Space.into(),
        c: ' ' as u32,
    };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\ ");
}

#[test]
fn test_fmt_literal_special_bell() {
    struct MockWriter {
        output: String,
    }
    
    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::Literal {
        kind: ast::SpecialLiteralKind::Bell.into(),
        c: '\x07' as u32,
    };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\a");
}

#[test]
fn test_fmt_literal_special_line_feed() {
    struct MockWriter {
        output: String,
    }
    
    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::Literal {
        kind: ast::SpecialLiteralKind::LineFeed.into(),
        c: '\n' as u32,
    };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\n");
}

#[test]
fn test_fmt_literal_special_form_feed() {
    struct MockWriter {
        output: String,
    }
    
    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::Literal {
        kind: ast::SpecialLiteralKind::FormFeed.into(),
        c: '\x0C' as u32,
    };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\f");
}

#[test]
fn test_fmt_literal_special_tab() {
    struct MockWriter {
        output: String,
    }
    
    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::Literal {
        kind: ast::SpecialLiteralKind::Tab.into(),
        c: '\t' as u32,
    };

    writer.fmt_literal(&ast).unwrap();
    assert_eq!(writer.output, r"\t");
}

