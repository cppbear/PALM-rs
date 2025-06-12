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

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder { ast: Ast::new() }
        }

        pub fn octal(&mut self, yes: bool) -> &mut ParserBuilder {
            self.ast.octal(yes);
            self
        }
    }

    let mut builder = ParserBuilder::new();
    let result = builder.octal(true);

    assert_eq!(result.ast.octal_enabled, true);
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

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder { ast: Ast::new() }
        }

        pub fn octal(&mut self, yes: bool) -> &mut ParserBuilder {
            self.ast.octal(yes);
            self
        }
    }

    let mut builder = ParserBuilder::new();
    let result = builder.octal(false);

    assert_eq!(result.ast.octal_enabled, false);
}

#[test]
#[should_panic]
fn test_octal_invalid() {
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

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder { ast: Ast::new() }
        }

        pub fn octal(&mut self, yes: bool) -> &mut ParserBuilder {
            self.ast.octal(yes);
            self
        }
    }

    let mut builder = ParserBuilder::new();
    builder.octal(true);
    // Simulating an invalid check that should panic
    assert_eq!(builder.ast.octal_enabled, false);
}

