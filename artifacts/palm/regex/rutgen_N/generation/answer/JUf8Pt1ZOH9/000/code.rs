// Answer 0

#[test]
fn test_cut_literal() {
    struct Literal {
        cut: bool,
    }

    impl Literal {
        fn new() -> Self {
            Literal { cut: false }
        }

        fn cut(&mut self) {
            self.cut = true;
        }
    }

    let mut literal = Literal::new();
    assert!(!literal.cut);
    literal.cut();
    assert!(literal.cut);
}

