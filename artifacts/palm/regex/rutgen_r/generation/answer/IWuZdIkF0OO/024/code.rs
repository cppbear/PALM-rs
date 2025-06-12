// Answer 0

#[test]
fn test_suffixes_unicode_class() {
    struct DummyLiterals {
        added: Vec<u8>,
        cut_called: bool,
    }

    impl DummyLiterals {
        fn new() -> Self {
            Self {
                added: Vec::new(),
                cut_called: false,
            }
        }

        fn cross_add(&mut self, buf: &[u8]) {
            self.added.extend_from_slice(buf);
        }

        fn add_char_class_reverse(&mut self, _cls: &str) -> bool {
            true // Simulate successful addition
        }

        fn cut(&mut self) {
            self.cut_called = true;
        }

        fn is_empty(&self) -> bool {
            self.added.is_empty()
        }
    }

    let expr = Hir::class_unicode("a-z"); // Assume `Hir::class_unicode` exists
    let mut lits = DummyLiterals::new();

    suffixes(&expr, &mut lits);

    assert!(!lits.is_empty());
    assert!(!lits.cut_called);
}

#[test]
fn test_suffixes_bytes_class() {
    struct DummyLiterals {
        added: Vec<u8>,
        cut_called: bool,
    }

    impl DummyLiterals {
        fn new() -> Self {
            Self {
                added: Vec::new(),
                cut_called: false,
            }
        }

        fn cross_add(&mut self, buf: &[u8]) {
            self.added.extend_from_slice(buf);
        }

        fn add_byte_class(&mut self, _cls: &[u8]) -> bool {
            true // Simulate successful addition
        }

        fn cut(&mut self) {
            self.cut_called = true;
        }

        fn is_empty(&self) -> bool {
            self.added.is_empty()
        }
    }

    let expr = Hir::class_bytes(&[b'a', b'b']); // Assume `Hir::class_bytes` exists
    let mut lits = DummyLiterals::new();

    suffixes(&expr, &mut lits);

    assert!(!lits.is_empty());
    assert!(!lits.cut_called);
}

#[test]
fn test_suffixes_empty_concat() {
    struct DummyLiterals {
        added: Vec<u8>,
        cut_called: bool,
    }

    impl DummyLiterals {
        fn new() -> Self {
            Self {
                added: Vec::new(),
                cut_called: false,
            }
        }

        fn cut(&mut self) {
            self.cut_called = true;
        }

        fn is_empty(&self) -> bool {
            self.added.is_empty()
        }
        
        fn to_empty(self) -> Self {
            Self::new()
        }
    }

    let expr = Hir::concat(&[]); // Assume `Hir::concat` exists and is empty
    let mut lits = DummyLiterals::new();

    suffixes(&expr, &mut lits);

    assert!(lits.is_empty());
    assert!(!lits.cut_called);
}

