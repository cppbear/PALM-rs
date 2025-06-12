// Answer 0

#[test]
fn test_octal_enable() {
    struct ParserBuilder {
        ast: Ast,
    }

    struct Ast {
        octal_enabled: bool,
    }

    impl Ast {
        fn new() -> Self {
            Ast { octal_enabled: false }
        }

        fn octal(&mut self, yes: bool) {
            self.octal_enabled = yes;
        }
    }

    let mut builder = ParserBuilder { ast: Ast::new() };
    builder.octal(true);
    assert!(builder.ast.octal_enabled);
}

#[test]
fn test_octal_disable() {
    struct ParserBuilder {
        ast: Ast,
    }

    struct Ast {
        octal_enabled: bool,
    }

    impl Ast {
        fn new() -> Self {
            Ast { octal_enabled: false }
        }

        fn octal(&mut self, yes: bool) {
            self.octal_enabled = yes;
        }
    }

    let mut builder = ParserBuilder { ast: Ast::new() };
    builder.octal(false);
    assert!(!builder.ast.octal_enabled);
}

