// Answer 0

#[derive(Debug)]
struct Literal {
    cut: bool,
}

impl Literal {
    pub fn new(cut: bool) -> Self {
        Literal { cut }
    }

    pub fn is_cut(&self) -> bool {
        self.cut
    }
}

#[test]
fn test_is_cut_true() {
    let lit = Literal::new(true);
    assert!(lit.is_cut());
}

#[test]
fn test_is_cut_false() {
    let lit = Literal::new(false);
    assert!(!lit.is_cut());
}

