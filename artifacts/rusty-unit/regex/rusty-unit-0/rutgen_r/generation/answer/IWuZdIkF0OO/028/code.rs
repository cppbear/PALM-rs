// Answer 0

#[test]
fn test_suffixes_unicode_literal() {
    struct TestLiterals {
        data: Vec<u8>,
    }

    impl TestLiterals {
        fn cross_add(&mut self, buf: &[u8]) {
            self.data.extend_from_slice(buf);
        }
        
        fn cut(&mut self) {
            self.data.clear();
        }
        
        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
        
        fn to_empty(&self) -> Self {
            TestLiterals { data: Vec::new() }
        }

        fn cross_product(&mut self, other: &Self) -> bool {
            !self.data.is_empty() && !other.data.is_empty()
        }

        fn add(&mut self, _: &str) {
            self.data.push(0); // Placeholder
        }

        fn any_complete(&self) -> bool {
            !self.data.is_empty()
        }
    }

    let unicode_literal = hir::Literal::Unicode('𝄞'); // Musical symbol G clef
    let expr = Hir::literal(unicode_literal);
    let mut lits = TestLiterals { data: Vec::new() };

    suffixes(&expr, &mut lits);
    assert!(!lits.is_empty());
}

#[test]
fn test_suffixes_byte_literal() {
    struct TestLiterals {
        data: Vec<u8>,
    }

    impl TestLiterals {
        fn cross_add(&mut self, buf: &[u8]) {
            self.data.extend_from_slice(buf);
        }
        
        fn cut(&mut self) {
            self.data.clear();
        }
        
        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
        
        fn to_empty(&self) -> Self {
            TestLiterals { data: Vec::new() }
        }

        fn cross_product(&mut self, other: &Self) -> bool {
            !self.data.is_empty() && !other.data.is_empty()
        }

        fn add(&mut self, _: &str) {
            self.data.push(0); // Placeholder
        }

        fn any_complete(&self) -> bool {
            !self.data.is_empty()
        }
    }

    let byte_literal = hir::Literal::Byte(b'a');
    let expr = Hir::literal(byte_literal);
    let mut lits = TestLiterals { data: Vec::new() };

    suffixes(&expr, &mut lits);
    assert!(!lits.is_empty());
}

