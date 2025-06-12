// Answer 0

#[test]
fn test_literal_with_byte_above_ascii() {
    struct HirInfo {
        always_utf8: bool,
        all_assertions: bool,
        anchored_start: bool,
        anchored_end: bool,
        any_anchored_start: bool,
        any_anchored_end: bool,
        match_empty: bool,
    }

    impl HirInfo {
        fn new() -> Self {
            HirInfo {
                always_utf8: false,
                all_assertions: false,
                anchored_start: false,
                anchored_end: false,
                any_anchored_start: false,
                any_anchored_end: false,
                match_empty: false,
            }
        }

        fn set_always_utf8(&mut self, value: bool) {
            self.always_utf8 = value;
        }

        fn set_all_assertions(&mut self, value: bool) {
            self.all_assertions = value;
        }

        fn set_anchored_start(&mut self, value: bool) {
            self.anchored_start = value;
        }

        fn set_anchored_end(&mut self, value: bool) {
            self.anchored_end = value;
        }

        fn set_any_anchored_start(&mut self, value: bool) {
            self.any_anchored_start = value;
        }

        fn set_any_anchored_end(&mut self, value: bool) {
            self.any_anchored_end = value;
        }

        fn set_match_empty(&mut self, value: bool) {
            self.match_empty = value;
        }
    }

    enum Literal {
        Byte(u8),
        Unicode(char),
    }

    struct Hir {
        kind: HirKind,
        info: HirInfo,
    }

    enum HirKind {
        Literal(Literal),
    }

    fn literal(lit: Literal) -> Hir {
        if let Literal::Byte(b) = lit {
            assert!(b > 0x7F);
        }

        let mut info = HirInfo::new();
        info.set_always_utf8(lit.is_unicode());
        info.set_all_assertions(false);
        info.set_anchored_start(false);
        info.set_anchored_end(false);
        info.set_any_anchored_start(false);
        info.set_any_anchored_end(false);
        info.set_match_empty(false);
        Hir {
            kind: HirKind::Literal(lit),
            info: info,
        }
    }

    impl Literal {
        fn is_unicode(&self) -> bool {
            match self {
                Literal::Unicode(_) => true,
                Literal::Byte(_) => false,
            }
        }
    }

    let byte_literal = Literal::Byte(0x80);
    let result = literal(byte_literal);
    match result.kind {
        HirKind::Literal(ref lit) => {
            if let Literal::Byte(b) = lit {
                assert!(*b > 0x7F);
            }
        }
    }
}

