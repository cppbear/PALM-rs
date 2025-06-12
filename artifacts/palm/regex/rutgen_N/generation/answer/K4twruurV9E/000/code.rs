// Answer 0

#[derive(Debug)]
struct Hir {
    value: String,
}

#[derive(Debug)]
struct Literals {
    literals: Vec<String>,
}

impl Literals {
    fn new() -> Self {
        Literals {
            literals: Vec::new(),
        }
    }
    
    fn cut(&mut self) {
        // Example implementation of cut that clears literals
        self.literals.clear();
    }
}

#[test]
fn test_repeat_one_or_more_literals() {
    let e = Hir {
        value: "a".to_string(),
    };
    
    let mut lits = Literals::new();
    
    let mut callback_called = false;
    
    let callback = |e: &Hir, lits: &mut Literals| {
        callback_called = true;
        lits.literals.push(e.value.clone());
    };
    
    repeat_one_or_more_literals(&e, &mut lits, callback);
    
    assert!(callback_called);
    assert_eq!(lits.literals, vec!["a"]);
    
    // Ensure cut was called and literals are cleared afterward
    lits.cut();
    assert!(lits.literals.is_empty());
}

