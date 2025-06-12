// Answer 0

#[test]
fn test_unicode_enable() {
    struct ParserBuilder {
        hir: Hir,
    }
    
    struct Hir {
        unicode_enabled: bool,
    }

    impl Hir {
        fn new() -> Self {
            Hir {
                unicode_enabled: true, // default is enabled
            }
        }
        
        fn unicode(&mut self, yes: bool) {
            self.unicode_enabled = yes;
        }
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                hir: Hir::new(),
            }
        }

        fn unicode(&mut self, yes: bool) -> &mut ParserBuilder {
            self.hir.unicode(yes);
            self
        }
    }

    let mut parser = ParserBuilder::new();
    let result = parser.unicode(true);
    assert_eq!(result.hir.unicode_enabled, true);

    let result = parser.unicode(false);
    assert_eq!(result.hir.unicode_enabled, false);
}

#[test]
fn test_unicode_disable() {
    struct ParserBuilder {
        hir: Hir,
    }
    
    struct Hir {
        unicode_enabled: bool,
    }

    impl Hir {
        fn new() -> Self {
            Hir {
                unicode_enabled: true, // default is enabled
            }
        }
        
        fn unicode(&mut self, yes: bool) {
            self.unicode_enabled = yes;
        }
    }

    impl ParserBuilder {
        fn new() -> Self {
            ParserBuilder {
                hir: Hir::new(),
            }
        }

        fn unicode(&mut self, yes: bool) -> &mut ParserBuilder {
            self.hir.unicode(yes);
            self
        }
    }

    let mut parser = ParserBuilder::new();
    parser.unicode(false);
    assert_eq!(parser.hir.unicode_enabled, false);
}

