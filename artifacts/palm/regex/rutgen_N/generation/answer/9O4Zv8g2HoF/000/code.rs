// Answer 0

#[test]
fn test_octal_enable() {
    struct ParserBuilder {
        octal: bool,
    }
    
    impl ParserBuilder {
        fn new() -> Self {
            Self { octal: false }
        }
        
        pub fn octal(&mut self, yes: bool) -> &mut Self {
            self.octal = yes;
            self
        }
    }

    let mut builder = ParserBuilder::new();
    builder.octal(true);
    assert!(builder.octal);
}

#[test]
fn test_octal_disable() {
    struct ParserBuilder {
        octal: bool,
    }
    
    impl ParserBuilder {
        fn new() -> Self {
            Self { octal: false }
        }
        
        pub fn octal(&mut self, yes: bool) -> &mut Self {
            self.octal = yes;
            self
        }
    }

    let mut builder = ParserBuilder::new();
    builder.octal(false);
    assert!(!builder.octal);
}

