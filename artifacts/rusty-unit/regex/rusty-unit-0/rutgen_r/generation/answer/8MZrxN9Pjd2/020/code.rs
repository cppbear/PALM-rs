// Answer 0

fn test_visit_pre_empty() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct Visitor<'a> {
        wtr: &'a mut MockWriter,
    }

    impl Visitor<'_> {
        fn visit_pre(&mut self, hir: &Hir) -> std::fmt::Result {
            match *hir.kind() {
                HirKind::Anchor(hir::Anchor::StartLine) => {
                    self.wtr.write_str("(?m:^)")?;
                }
                HirKind::Anchor(hir::Anchor::EndLine) => {
                    self.wtr.write_str("(?m:$)")?;
                }
                HirKind::Anchor(hir::Anchor::StartText) => {
                    self.wtr.write_str(r"\A")?;
                }
                HirKind::Anchor(hir::Anchor::EndText) => {
                    self.wtr.write_str(r"\z")?;
                }
                HirKind::Class(hir::Class::Unicode(ref cls)) => {
                    self.wtr.write_str("[")?;
                    for range in cls.iter() {
                        if range.start() == range.end() {
                            self.wtr.write_str(&format!("{}", range.start()))?;
                        } else {
                            self.wtr.write_str(&format!("{}", range.start()))?;
                            self.wtr.write_str("-")?;
                            self.wtr.write_str(&format!("{}", range.end()))?;
                        }
                    }
                    self.wtr.write_str("]")?;
                }
                HirKind::Class(hir::Class::Bytes(ref cls)) => {
                    self.wtr.write_str("(?-u:[")?;
                    for range in cls.iter() {
                        if range.start() == range.end() {
                            self.wtr.write_str(&format!("{}", range.start()))?;
                        } else {
                            self.wtr.write_str(&format!("{}", range.start()))?;
                            self.wtr.write_str("-")?;
                            self.wtr.write_str(&format!("{}", range.end()))?;
                        }
                    }
                    self.wtr.write_str("])")?;
                }
                _ => {}
            }
            Ok(())
        }
    }

    let mut mock_writer = MockWriter::new();
    let mut visitor = Visitor { wtr: &mut mock_writer };

    let hir_start_line = Hir::from(HirKind::Anchor(hir::Anchor::StartLine));
    let result_start_line = visitor.visit_pre(&hir_start_line);
    
    assert!(result_start_line.is_ok());
    assert_eq!(mock_writer.output, "(?m:^)");
    
    let hir_start_text = Hir::from(HirKind::Anchor(hir::Anchor::StartText));
    let result_start_text = visitor.visit_pre(&hir_start_text);
    
    assert!(result_start_text.is_ok());
    assert_eq!(mock_writer.output, "(?m:^)\\A");

    let bytes_class = Hir::from(HirKind::Class(hir::Class::Bytes(vec![1..=2, 3..=4])));
    let result_bytes_class = visitor.visit_pre(&bytes_class);
    
    assert!(result_bytes_class.is_ok());
    assert_eq!(mock_writer.output, "(?m:^)\\A(?-u:[1-2][3-4])");
}

