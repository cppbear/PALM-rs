// Answer 0

#[test]
fn test_literal_to_char_unicode_mode() {
    struct TestStruct {
        flags: fn() -> Flags,
    }

    struct Flags {
        unicode_mode: bool,
    }

    impl TestStruct {
        fn flags(&self) -> Flags {
            (self.flags)()
        }
    }

    struct AstLiteral {
        c: char,
    }

    impl AstLiteral {
        fn byte(&self) -> Option<u8> {
            None
        }
    }

    struct HirLiteral {
        value: hir::Literal,
    }

    let test_struct = TestStruct {
        flags: || Flags { unicode_mode: true },
    };

    let lit = AstLiteral { c: 'a' };
    let result = test_struct.literal_to_char(&lit);

    assert_eq!(result.unwrap(), HirLiteral { value: hir::Literal::Unicode('a') });
}

#[test]
fn test_literal_to_char_ascii_value() {
    struct TestStruct {
        flags: fn() -> Flags,
        trans: fn() -> Trans,
    }

    struct Flags {
        unicode_mode: bool,
    }

    struct Trans {
        allow_invalid_utf8: bool,
    }

    impl TestStruct {
        fn flags(&self) -> Flags {
            (self.flags)()
        }

        fn trans(&self) -> Trans {
            (self.trans)()
        }
    }

    struct AstLiteral {
        c: char,
    }

    impl AstLiteral {
        fn byte(&self) -> Option<u8> {
            Some(97) // ASCII 'a'
        }
    }

    struct HirLiteral {
        value: hir::Literal,
    }

    let test_struct = TestStruct {
        flags: || Flags { unicode_mode: false },
        trans: || Trans { allow_invalid_utf8: true },
    };

    let lit = AstLiteral { c: 'a' };
    let result = test_struct.literal_to_char(&lit);

    assert_eq!(result.unwrap(), HirLiteral { value: hir::Literal::Unicode('a') });
}

#[test]
fn test_literal_to_char_invalid_utf8() {
    struct TestStruct {
        flags: fn() -> Flags,
        trans: fn() -> Trans,
    }

    struct Flags {
        unicode_mode: bool,
    }

    struct Trans {
        allow_invalid_utf8: bool,
    }

    impl TestStruct {
        fn flags(&self) -> Flags {
            (self.flags)()
        }

        fn trans(&self) -> Trans {
            (self.trans)()
        }
    }

    struct AstLiteral {
        c: char,
    }

    impl AstLiteral {
        fn byte(&self) -> Option<u8> {
            Some(200) // Non-ASCII byte
        }
    }

    let test_struct = TestStruct {
        flags: || Flags { unicode_mode: false },
        trans: || Trans { allow_invalid_utf8: false },
    };

    let lit = AstLiteral { c: 'a' };
    let result = test_struct.literal_to_char(&lit);

    assert!(result.is_err());
}

