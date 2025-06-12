// Answer 0

#[test]
fn test_suffixes_unicode_literal() {
    struct DummyLiterals {
        data: Vec<u8>,
    }
    
    impl DummyLiterals {
        fn new() -> Self {
            DummyLiterals { data: Vec::new() }
        }

        fn cross_add(&mut self, buf: &[u8]) {
            self.data.extend_from_slice(buf);
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn to_empty(&mut self) -> Self {
            let temp = self.data.clone();
            self.data.clear();
            DummyLiterals { data: temp }
        }

        fn cross_product(&self, _other: &Self) -> bool {
            true
        }

        fn any_complete(&self) -> bool {
            true
        }
    }

    let expr = Hir::new_literal(hir::Literal::Unicode('a'));
    let mut lits = DummyLiterals::new();
    suffixes(&expr, &mut lits);
    assert!(!lits.is_empty());
}

#[test]
fn test_suffixes_byte_literal() {
    struct DummyLiterals {
        data: Vec<u8>,
    }
    
    impl DummyLiterals {
        fn new() -> Self {
            DummyLiterals { data: Vec::new() }
        }

        fn cross_add(&mut self, buf: &[u8]) {
            self.data.extend_from_slice(buf);
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn to_empty(&mut self) -> Self {
            let temp = self.data.clone();
            self.data.clear();
            DummyLiterals { data: temp }
        }

        fn cross_product(&self, _other: &Self) -> bool {
            true
        }

        fn any_complete(&self) -> bool {
            true
        }
    }

    let expr = Hir::new_literal(hir::Literal::Byte(b'a'));
    let mut lits = DummyLiterals::new();
    suffixes(&expr, &mut lits);
    assert!(!lits.is_empty());
}

#[test]
fn test_suffixes_class_unicode() {
    struct DummyLiterals {
        data: Vec<u8>,
    }
    
    impl DummyLiterals {
        fn new() -> Self {
            DummyLiterals { data: Vec::new() }
        }

        fn cross_add(&mut self, _buf: &[u8]) {
            // Simulate adding literal for class
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn add_char_class_reverse(&self, _cls: &hir::Class) -> bool {
            true
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn cross_product(&self, _other: &Self) -> bool {
            true
        }

        fn any_complete(&self) -> bool {
            true
        }

        fn to_empty(&mut self) -> Self {
            let temp = self.data.clone();
            self.data.clear();
            DummyLiterals { data: temp }
        }
    }

    let expr = Hir::new_class(hir::Class::Unicode(vec!['a', 'b', 'c']));
    let mut lits = DummyLiterals::new();
    suffixes(&expr, &mut lits);
    assert!(!lits.is_empty());
}

#[test]
fn test_suffixes_group() {
    struct DummyLiterals {
        data: Vec<u8>,
    }
    
    impl DummyLiterals {
        fn new() -> Self {
            DummyLiterals { data: Vec::new() }
        }

        fn cross_add(&mut self, _buf: &[u8]) {
            // Simulate adding literal for group
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn to_empty(&mut self) -> Self {
            let temp = self.data.clone();
            self.data.clear();
            DummyLiterals { data: temp }
        }

        fn cross_product(&self, _other: &Self) -> bool {
            true
        }

        fn any_complete(&self) -> bool {
            true
        }
    }

    let inner_expr = Hir::new_literal(hir::Literal::Unicode('a'));
    let expr = Hir::new_group(inner_expr);
    let mut lits = DummyLiterals::new();
    suffixes(&expr, &mut lits);
    assert!(!lits.is_empty());
}

#[test]
fn test_suffixes_concat() {
    struct DummyLiterals {
        data: Vec<u8>,
    }
    
    impl DummyLiterals {
        fn new() -> Self {
            DummyLiterals { data: Vec::new() }
        }

        fn cross_add(&mut self, _buf: &[u8]) {
            // Simulate adding literal for concat
        }

        fn cut(&mut self) {
            self.data.clear();
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn to_empty(&mut self) -> Self {
            let temp = self.data.clone();
            self.data.clear();
            DummyLiterals { data: temp }
        }

        fn cross_product(&self, _other: &Self) -> bool {
            true
        }

        fn any_complete(&self) -> bool {
            true
        }
    }

    let expr1 = Hir::new_literal(hir::Literal::Unicode('a'));
    let expr2 = Hir::new_literal(hir::Literal::Unicode('b'));
    let expr = Hir::new_concat(vec![expr1, expr2]);
    let mut lits = DummyLiterals::new();
    suffixes(&expr, &mut lits);
    assert!(!lits.is_empty());
}

