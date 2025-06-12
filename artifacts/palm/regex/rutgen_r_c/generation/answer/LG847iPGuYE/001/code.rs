// Answer 0

#[test]
fn test_visit_post_with_alternation() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let alternation_hir = Hir {
        kind: HirKind::Alternation(vec![]),
        info: HirInfo {},
    };

    let result = visitor.visit_post(&alternation_hir);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_with_anchor() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let anchor_hir = Hir {
        kind: HirKind::Anchor(Anchor {}),
        info: HirInfo {},
    };

    let result = visitor.visit_post(&anchor_hir);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_with_empty() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let empty_hir = Hir {
        kind: HirKind::Empty,
        info: HirInfo {},
    };

    let result = visitor.visit_post(&empty_hir);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_with_class() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let class_hir = Hir {
        kind: HirKind::Class(Class {}),
        info: HirInfo {},
    };

    let result = visitor.visit_post(&class_hir);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_with_literal() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let literal_hir = Hir {
        kind: HirKind::Literal(Literal {}),
        info: HirInfo {},
    };

    let result = visitor.visit_post(&literal_hir);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_with_concat() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let concat_hir = Hir {
        kind: HirKind::Concat(vec![]),
        info: HirInfo {},
    };

    let result = visitor.visit_post(&concat_hir);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_visit_post_with_word_boundary() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: writer };

    let word_boundary_hir = Hir {
        kind: HirKind::WordBoundary(WordBoundary {}),
        info: HirInfo {},
    };

    let result = visitor.visit_post(&word_boundary_hir);
    assert_eq!(result, Ok(()));
}

