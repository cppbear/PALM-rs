// Answer 0

#[test]
fn test_octal_enable() {
    struct ParserBuilder {
        octal: bool,
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder { octal: false }
        }

        fn octal(&mut self, yes: bool) -> &mut ParserBuilder {
            self.octal = yes;
            self
        }
    }

    let mut parser = ParserBuilder::new();
    
    let result = parser.octal(true);
    
    assert_eq!(result.octal, true);
}

#[test]
fn test_octal_disable() {
    struct ParserBuilder {
        octal: bool,
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder { octal: true }
        }

        fn octal(&mut self, yes: bool) -> &mut ParserBuilder {
            self.octal = yes;
            self
        }
    }

    let mut parser = ParserBuilder::new();
    
    let result = parser.octal(false);
    
    assert_eq!(result.octal, false);
}

