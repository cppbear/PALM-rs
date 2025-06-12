// Answer 0

#[test]
fn test_fmt_class_perl_digit() {
    struct MockWriter {
        output: String,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }
        
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    struct Formatter {
        wtr: MockWriter,
    }
    
    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: MockWriter::new(),
            }
        }
        
        fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {
            use ast::ClassPerlKind::*;
            match ast.kind {
                Digit if ast.negated => self.wtr.write_str(r"\D"),
                Digit => self.wtr.write_str(r"\d"),
                Space if ast.negated => self.wtr.write_str(r"\S"),
                Space => self.wtr.write_str(r"\s"),
                Word if ast.negated => self.wtr.write_str(r"\W"),
                Word => self.wtr.write_str(r"\w"),
            }
        }
    }
    
    let ast = ast::ClassPerl {
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };
    
    let mut formatter = Formatter::new();
    formatter.fmt_class_perl(&ast).unwrap();
    
    assert_eq!(formatter.wtr.output, r"\d");
}

#[test]
fn test_fmt_class_perl_digit_negated() {
    struct MockWriter {
        output: String,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }
        
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    struct Formatter {
        wtr: MockWriter,
    }
    
    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: MockWriter::new(),
            }
        }
        
        fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {
            use ast::ClassPerlKind::*;
            match ast.kind {
                Digit if ast.negated => self.wtr.write_str(r"\D"),
                Digit => self.wtr.write_str(r"\d"),
                Space if ast.negated => self.wtr.write_str(r"\S"),
                Space => self.wtr.write_str(r"\s"),
                Word if ast.negated => self.wtr.write_str(r"\W"),
                Word => self.wtr.write_str(r"\w"),
            }
        }
    }
    
    let ast = ast::ClassPerl {
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    };
    
    let mut formatter = Formatter::new();
    formatter.fmt_class_perl(&ast).unwrap();
    
    assert_eq!(formatter.wtr.output, r"\D");
} 

#[test]
fn test_fmt_class_perl_space() {
    struct MockWriter {
        output: String,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }
        
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    struct Formatter {
        wtr: MockWriter,
    }
    
    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: MockWriter::new(),
            }
        }
        
        fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {
            use ast::ClassPerlKind::*;
            match ast.kind {
                Digit if ast.negated => self.wtr.write_str(r"\D"),
                Digit => self.wtr.write_str(r"\d"),
                Space if ast.negated => self.wtr.write_str(r"\S"),
                Space => self.wtr.write_str(r"\s"),
                Word if ast.negated => self.wtr.write_str(r"\W"),
                Word => self.wtr.write_str(r"\w"),
            }
        }
    }
    
    let ast = ast::ClassPerl {
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };
    
    let mut formatter = Formatter::new();
    formatter.fmt_class_perl(&ast).unwrap();
    
    assert_eq!(formatter.wtr.output, r"\s");
}

#[test]
fn test_fmt_class_perl_space_negated() {
    struct MockWriter {
        output: String,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }
        
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    struct Formatter {
        wtr: MockWriter,
    }
    
    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: MockWriter::new(),
            }
        }
        
        fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {
            use ast::ClassPerlKind::*;
            match ast.kind {
                Digit if ast.negated => self.wtr.write_str(r"\D"),
                Digit => self.wtr.write_str(r"\d"),
                Space if ast.negated => self.wtr.write_str(r"\S"),
                Space => self.wtr.write_str(r"\s"),
                Word if ast.negated => self.wtr.write_str(r"\W"),
                Word => self.wtr.write_str(r"\w"),
            }
        }
    }
    
    let ast = ast::ClassPerl {
        kind: ast::ClassPerlKind::Space,
        negated: true,
    };
    
    let mut formatter = Formatter::new();
    formatter.fmt_class_perl(&ast).unwrap();
    
    assert_eq!(formatter.wtr.output, r"\S");
}

#[test]
fn test_fmt_class_perl_word() {
    struct MockWriter {
        output: String,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }
        
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    struct Formatter {
        wtr: MockWriter,
    }
    
    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: MockWriter::new(),
            }
        }
        
        fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {
            use ast::ClassPerlKind::*;
            match ast.kind {
                Digit if ast.negated => self.wtr.write_str(r"\D"),
                Digit => self.wtr.write_str(r"\d"),
                Space if ast.negated => self.wtr.write_str(r"\S"),
                Space => self.wtr.write_str(r"\s"),
                Word if ast.negated => self.wtr.write_str(r"\W"),
                Word => self.wtr.write_str(r"\w"),
            }
        }
    }
    
    let ast = ast::ClassPerl {
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };
    
    let mut formatter = Formatter::new();
    formatter.fmt_class_perl(&ast).unwrap();
    
    assert_eq!(formatter.wtr.output, r"\w");
}

#[test]
fn test_fmt_class_perl_word_negated() {
    struct MockWriter {
        output: String,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }
        
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    struct Formatter {
        wtr: MockWriter,
    }
    
    impl Formatter {
        fn new() -> Self {
            Formatter {
                wtr: MockWriter::new(),
            }
        }
        
        fn fmt_class_perl(&mut self, ast: &ast::ClassPerl) -> fmt::Result {
            use ast::ClassPerlKind::*;
            match ast.kind {
                Digit if ast.negated => self.wtr.write_str(r"\D"),
                Digit => self.wtr.write_str(r"\d"),
                Space if ast.negated => self.wtr.write_str(r"\S"),
                Space => self.wtr.write_str(r"\s"),
                Word if ast.negated => self.wtr.write_str(r"\W"),
                Word => self.wtr.write_str(r"\w"),
            }
        }
    }
    
    let ast = ast::ClassPerl {
        kind: ast::ClassPerlKind::Word,
        negated: true,
    };
    
    let mut formatter = Formatter::new();
    formatter.fmt_class_perl(&ast).unwrap();
    
    assert_eq!(formatter.wtr.output, r"\W");
}

