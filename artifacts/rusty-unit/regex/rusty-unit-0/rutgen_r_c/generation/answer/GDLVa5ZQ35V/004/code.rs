// Answer 0

#[test]
fn test_visit_post_repetition_greedy() {
    struct MockPrinter {
        output: String,
    }
    
    impl fmt::Write for MockPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut printer = MockPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut printer, wtr: &mut printer };
    
    let repetition_ast = Repetition {
        span: Span::default(),
        op: RepetitionOp::one_or_more(true),
        greedy: true,
        ast: Box::new(Ast::Empty(Span::default())),
    };

    let result = writer.visit_post(&Ast::Repetition(repetition_ast));
    
    assert!(result.is_ok());
    assert_eq!(printer.output, "+");
}

#[test]
fn test_visit_post_repetition_non_greedy() {
    struct MockPrinter {
        output: String,
    }
    
    impl fmt::Write for MockPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut printer = MockPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut printer, wtr: &mut printer };
    
    let repetition_ast = Repetition {
        span: Span::default(),
        op: RepetitionOp::zero_or_more(false),
        greedy: false,
        ast: Box::new(Ast::Empty(Span::default())),
    };

    let result = writer.visit_post(&Ast::Repetition(repetition_ast));
    
    assert!(result.is_ok());
    assert_eq!(printer.output, "*?");
}

#[test]
fn test_visit_post_repetition_range() {
    struct MockPrinter {
        output: String,
    }
    
    impl fmt::Write for MockPrinter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut printer = MockPrinter { output: String::new() };
    let mut writer = Writer { printer: &mut printer, wtr: &mut printer };
    
    let repetition_ast = Repetition {
        span: Span::default(),
        op: RepetitionOp::range(2, 5, true),
        greedy: true,
        ast: Box::new(Ast::Empty(Span::default())),
    };

    let result = writer.visit_post(&Ast::Repetition(repetition_ast));
    
    assert!(result.is_ok());
    // Assuming fmt_repetition_range writes appropriate range expression (not implemented here)
}

