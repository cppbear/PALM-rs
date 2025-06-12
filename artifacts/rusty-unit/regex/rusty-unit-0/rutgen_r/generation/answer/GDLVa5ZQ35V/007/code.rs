// Answer 0

fn test_visit_post_empty() {
    use regex_syntax::ast::{Ast, Class};
    
    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
        
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self { wtr: TestWriter::new() }
        }

        fn fmt_set_flags(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_literal(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_assertion(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_class_perl(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_class_unicode(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_class_bracketed_post(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_repetition(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_group_post(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
    }

    let mut visitor = TestVisitor::new();
    let ast = Ast::Empty(());
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
}

fn test_visit_post_unicode() {
    use regex_syntax::ast::{Ast, Class};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
        
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self { wtr: TestWriter::new() }
        }

        fn fmt_set_flags(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_literal(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_assertion(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_class_perl(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_class_unicode(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_class_bracketed_post(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_repetition(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_group_post(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
    }

    let mut visitor = TestVisitor::new();
    let ast = Ast::Class(Class::Unicode(()));
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
}

fn test_visit_post_bracketed() {
    use regex_syntax::ast::{Ast, Class};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
        
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self { wtr: TestWriter::new() }
        }

        fn fmt_set_flags(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_literal(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_assertion(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_class_perl(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_class_unicode(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_class_bracketed_post(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_repetition(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_group_post(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
    }

    let mut visitor = TestVisitor::new();
    let ast = Ast::Class(Class::Bracketed(()));
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
}

fn test_visit_post_perl() {
    use regex_syntax::ast::{Ast, Class};

    struct TestWriter {
        output: String,
    }

    impl TestWriter {
        fn new() -> Self {
            Self { output: String::new() }
        }
        
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestVisitor {
        wtr: TestWriter,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self { wtr: TestWriter::new() }
        }

        fn fmt_set_flags(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_literal(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_assertion(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_class_perl(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_class_unicode(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_class_bracketed_post(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_repetition(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
        fn fmt_group_post(&mut self, _: &()) -> std::fmt::Result { Ok(()) }
    }

    let mut visitor = TestVisitor::new();
    let ast = Ast::Class(Class::Perl(()));
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
}

