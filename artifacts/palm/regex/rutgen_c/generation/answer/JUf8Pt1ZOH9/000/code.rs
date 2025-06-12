// Answer 0

#[test]
fn test_cut_sets_cut_to_true() {
    struct TestLiteral {
        v: Vec<u8>,
        cut: bool,
    }
    
    impl Literal {
        pub fn new(bytes: Vec<u8>) -> Self {
            Self { v: bytes, cut: false }
        }
        
        pub fn is_cut(&self) -> bool {
            self.cut
        }

        pub fn cut(&mut self) {
            self.cut = true;
        }
    }

    let mut literal = TestLiteral::new(vec![b'a', b'b', b'c']);
    assert!(!literal.is_cut());
    literal.cut();
    assert!(literal.is_cut());
}

#[test]
fn test_cut_on_empty_literal() {
    struct TestLiteral {
        v: Vec<u8>,
        cut: bool,
    }
    
    impl Literal {
        pub fn new(bytes: Vec<u8>) -> Self {
            Self { v: bytes, cut: false }
        }
        
        pub fn is_cut(&self) -> bool {
            self.cut
        }
        
        pub fn cut(&mut self) {
            self.cut = true;
        }
    }

    let mut literal = TestLiteral::new(vec![]);
    assert!(!literal.is_cut());
    literal.cut();
    assert!(literal.is_cut());
}

