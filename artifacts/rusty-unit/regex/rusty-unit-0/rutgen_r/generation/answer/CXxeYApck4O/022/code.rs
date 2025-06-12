// Answer 0

#[test]
fn test_prefixes_with_unicode_literal() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new_unicode_literal(c: char) -> Self {
            Self { kind: HirKind::Literal(hir::Literal::Unicode(c)) }
        }
    }

    let mut literals = Literals::new();
    let expr = TestHir::new_unicode_literal('a');
    prefixes(&expr, &mut literals);
    assert!(!literals.is_empty());
}

#[test]
fn test_prefixes_with_byte_literal() {
    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new_byte_literal(b: u8) -> Self {
            Self { kind: HirKind::Literal(hir::Literal::Byte(b)) }
        }
    }

    let mut literals = Literals::new();
    let expr = TestHir::new_byte_literal(97); // ASCII for 'a'
    prefixes(&expr, &mut literals);
    assert!(!literals.is_empty());
}

#[test]
fn test_prefixes_with_unicode_class() {
    struct TestClass {
        cls: hir::Class,
    }

    impl TestClass {
        fn new_unicode_class(chars: &[char]) -> hir::Class {
            hir::Class::Unicode(hir::UnicodeClass::new(chars))
        }
    }

    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new_unicode_class(cls: &hir::Class) -> Self {
            Self { kind: HirKind::Class(cls.clone()) }
        }
    }

    let mut literals = Literals::new();
    let unicode_class = TestClass::new_unicode_class(&['a', 'b', 'c']);
    let expr = TestHir::new_unicode_class(&unicode_class);
    prefixes(&expr, &mut literals);
    assert!(literals.add_char_class(&unicode_class));
}

#[test]
fn test_prefixes_with_byte_class() {
    struct TestClass {
        cls: hir::Class,
    }

    impl TestClass {
        fn new_byte_class(bytes: &[u8]) -> hir::Class {
            hir::Class::Bytes(hir::ByteClass::new(bytes))
        }
    }

    struct TestHir {
        kind: HirKind,
    }

    impl TestHir {
        fn new_byte_class(cls: &hir::Class) -> Self {
            Self { kind: HirKind::Class(cls.clone()) }
        }
    }

    let mut literals = Literals::new();
    let byte_class = TestClass::new_byte_class(&[97, 98, 99]); // ASCII for 'a', 'b', 'c'
    let expr = TestHir::new_byte_class(&byte_class);
    prefixes(&expr, &mut literals);
    assert!(literals.add_byte_class(&byte_class));
}

