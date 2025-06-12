// Answer 0

fn visit_pre_test() -> fmt::Result {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockVisitor {
        wtr: MockWriter,
    }
    
    impl MockVisitor {
        fn new() -> Self {
            MockVisitor {
                wtr: MockWriter {
                    output: String::new(),
                },
            }
        }

        fn write_literal_char(&mut self, _c: char) -> fmt::Result {
            Ok(())
        }

        fn write_literal_byte(&mut self, _b: u8) -> fmt::Result {
            Ok(())
        }

        fn write_literal_class_byte(&mut self, _b: u8) -> fmt::Result {
            Ok(())
        }
    }

    enum HirKind {
        WordBoundary(hir::WordBoundary),
        Class(hir::Class),
    }

    mod hir {
        pub enum WordBoundary {
            UnicodeNegate,
            Ascii,
            AsciiNegate,
            Unicode,
        }

        pub enum Class {
            Bytes(Vec<u8>),
            Unicode(Vec<char>),
        }
    }

    struct Hir {
        kind: HirKind,
    }

    impl Hir {
        fn kind(&self) -> &HirKind {
            &self.kind
        }
    }

    let mut visitor = MockVisitor::new();
    let hir = Hir {
        kind: HirKind::WordBoundary(hir::WordBoundary::Ascii),
    };

    assert_eq!(visitor.visit_pre(&hir), Ok(()));
    
    let hir_class_unicode = Hir {
        kind: HirKind::Class(hir::Class::Unicode(vec!['a', 'b', 'c'])),
    };

    assert_eq!(visitor.visit_pre(&hir_class_unicode), Ok(()));
    
    let hir_class_bytes = Hir {
        kind: HirKind::Class(hir::Class::Bytes(vec![1, 2, 3])),
    };

    assert_eq!(visitor.visit_pre(&hir_class_bytes), Ok(()));
    
    let hir_word_boundary_unicode_negate = Hir {
        kind: HirKind::WordBoundary(hir::WordBoundary::UnicodeNegate),
    };

    assert_eq!(visitor.visit_pre(&hir_word_boundary_unicode_negate), Ok(()));
    
    let hir_word_boundary_unicode = Hir {
        kind: HirKind::WordBoundary(hir::WordBoundary::Unicode),
    };

    assert_eq!(visitor.visit_pre(&hir_word_boundary_unicode), Ok(()));
    
    Ok(())
}

