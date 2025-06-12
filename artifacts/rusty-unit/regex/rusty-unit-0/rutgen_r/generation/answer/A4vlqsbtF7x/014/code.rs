// Answer 0

#[derive(Clone, Debug, PartialEq)]
struct Literal {
    len: usize,
    cut: bool,
}

impl Literal {
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn cut(&mut self) {
        self.cut = true;
    }

    fn truncate(&mut self, _i: usize) {
        self.len = 0;
    }
}

#[derive(Debug)]
struct Literals {
    lits: Vec<Literal>,
}

impl Literals {
    fn to_empty(&self) -> Self {
        Literals { lits: Vec::new() }
    }

    fn new(lits: Vec<Literal>) -> Self {
        Literals { lits }
    }
}

fn position(_lit1: &Literal, _lit2: &Literal) -> Option<usize> {
    Some(0) // Dummy implementation for the sake of the test
}

#[test]
fn test_unambiguous_prefixes() {
    let lit1 = Literal { len: 3, cut: false };
    let lit2 = Literal { len: 3, cut: false };
    let lit3 = Literal { len: 4, cut: false };
    let lit4 = Literal { len: 5, cut: false };

    let literals = Literals::new(vec![lit1.clone(), lit2.clone(), lit3.clone(), lit4.clone()]);
    
    let result = literals.unambiguous_prefixes();
    
    assert_eq!(result.lits.len(), 3);
    assert!(result.lits.contains(&lit3));
    assert!(result.lits.contains(&lit4));

    let lit5 = Literal { len: 5, cut: false };
    let literals2 = Literals::new(vec![lit1, lit5]);

    let result2 = literals2.unambiguous_prefixes();

    assert_eq!(result2.lits.len(), 2);
    assert!(result2.lits.contains(&lit5));
}

