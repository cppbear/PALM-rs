// Answer 0

#[test]
fn test_class_with_utf8() {
    struct MockClass {
        always_utf8: bool,
    }

    impl MockClass {
        fn is_always_utf8(&self) -> bool {
            self.always_utf8
        }
    }

    let class_instance = MockClass { always_utf8: true };
    let result = regex_syntax::class(class_instance);

    assert_eq!(result.kind, regex_syntax::HirKind::Class(class_instance));
    assert!(result.info.always_utf8());
}

#[test]
fn test_class_without_utf8() {
    struct MockClass {
        always_utf8: bool,
    }

    impl MockClass {
        fn is_always_utf8(&self) -> bool {
            self.always_utf8
        }
    }

    let class_instance = MockClass { always_utf8: false };
    let result = regex_syntax::class(class_instance);

    assert_eq!(result.kind, regex_syntax::HirKind::Class(class_instance));
    assert!(!result.info.always_utf8());
}

#[test]
fn test_class_info_initialization() {
    struct MockClass {
        always_utf8: bool,
    }

    impl MockClass {
        fn is_always_utf8(&self) -> bool {
            self.always_utf8
        }
    }

    let class_instance = MockClass { always_utf8: false };
    let result = regex_syntax::class(class_instance);

    assert_eq!(result.info.all_assertions(), false);
    assert_eq!(result.info.anchored_start(), false);
    assert_eq!(result.info.anchored_end(), false);
    assert_eq!(result.info.any_anchored_start(), false);
    assert_eq!(result.info.any_anchored_end(), false);
    assert_eq!(result.info.match_empty(), false);
}

