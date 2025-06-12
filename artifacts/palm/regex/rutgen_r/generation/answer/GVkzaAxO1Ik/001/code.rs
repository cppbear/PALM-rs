// Answer 0

#[test]
fn test_cut_with_non_empty_lits() {
    struct Lit {
        cut_called: bool,
    }

    impl Lit {
        fn new() -> Self {
            Lit { cut_called: false }
        }
        
        fn cut(&mut self) {
            self.cut_called = true;
        }
    }

    struct LiteralSet {
        lits: Vec<Lit>,
    }

    impl LiteralSet {
        fn new(lits: Vec<Lit>) -> Self {
            LiteralSet { lits }
        }
        
        pub fn cut(&mut self) {
            for lit in &mut self.lits {
                lit.cut();
            }
        }
    }

    let mut set = LiteralSet::new(vec![Lit::new(), Lit::new()]);
    set.cut();

    assert!(set.lits[0].cut_called);
    assert!(set.lits[1].cut_called);
}

#[test]
fn test_cut_with_empty_lits() {
    struct Lit {
        cut_called: bool,
    }

    impl Lit {
        fn new() -> Self {
            Lit { cut_called: false }
        }
        
        fn cut(&mut self) {
            self.cut_called = true;
        }
    }

    struct LiteralSet {
        lits: Vec<Lit>,
    }

    impl LiteralSet {
        fn new(lits: Vec<Lit>) -> Self {
            LiteralSet { lits }
        }
        
        pub fn cut(&mut self) {
            for lit in &mut self.lits {
                lit.cut();
            }
        }
    }

    let mut set = LiteralSet::new(vec![]);
    set.cut();

    assert!(set.lits.is_empty());
}

