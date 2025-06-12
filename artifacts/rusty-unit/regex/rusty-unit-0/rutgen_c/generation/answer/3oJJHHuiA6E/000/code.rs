// Answer 0

#[test]
fn test_class_unicode() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassUnicode; // Minimal struct for testing

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassBytes {
        // Minimal ClassBytes struct for testing
        is_all_ascii: bool,
    }

    impl ClassBytes {
        fn is_all_ascii(&self) -> bool {
            self.is_all_ascii
        }
    }

    let unicode_class = Class::Unicode(ClassUnicode);
    let hir = Hir::class(unicode_class);
    assert!(hir.is_always_utf8());
}

#[test]
fn test_class_bytes() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassBytes {
        is_all_ascii: bool,
    }

    impl ClassBytes {
        fn is_all_ascii(&self) -> bool {
            self.is_all_ascii
        }
    }

    let bytes_class = Class::Bytes(ClassBytes { is_all_ascii: true });
    let hir = Hir::class(bytes_class);
    assert!(hir.is_always_utf8());

    let bytes_class_non_ascii = Class::Bytes(ClassBytes { is_all_ascii: false });
    let hir_non_ascii = Hir::class(bytes_class_non_ascii);
    assert!(!hir_non_ascii.is_always_utf8());
}

#[test]
fn test_class_info_defaults() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassUnicode; // Minimal struct for testing

    let unicode_class = Class::Unicode(ClassUnicode);
    let hir = Hir::class(unicode_class);
    let info = hir.info;

    assert!(!info.is_all_assertions());
    assert!(!info.is_anchored_start());
    assert!(!info.is_anchored_end());
    assert!(!info.is_any_anchored_start());
    assert!(!info.is_any_anchored_end());
    assert!(!info.is_match_empty());
}

