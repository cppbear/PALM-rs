// Answer 0

#[derive(Debug)]
struct Hir {
    value: String,
}

#[derive(Debug)]
struct Literals {
    limit_size: usize,
    content: Vec<String>,
}

impl Literals {
    fn new(limit_size: usize) -> Self {
        Literals {
            limit_size,
            content: Vec::new(),
        }
    }

    fn to_empty(&self) -> Literals {
        Literals::new(self.limit_size)
    }

    fn set_limit_size(&mut self, size: usize) {
        self.limit_size = size;
    }

    fn limit_size(&self) -> usize {
        self.limit_size
    }

    fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    fn union(&mut self, other: &Literals) -> bool {
        if self.content.len() + other.content.len() <= self.limit_size {
            self.content.extend(other.content.clone());
            true
        } else {
            false
        }
    }

    fn cut(&mut self) {
        self.content.clear();
    }

    fn cross_product(&self, other: &Literals) -> bool {
        self.content.len() * other.content.len() <= self.limit_size
    }
}

#[test]
fn test_alternate_literals_nofailure() {
    let literals = Literals::new(15);
    let mut unioned_literals = literals.to_empty();
    unioned_literals.content.push("lit1".to_string());
    unioned_literals.content.push("lit2".to_string());

    let es = vec![
        Hir { value: "h1".to_string() },
        Hir { value: "h2".to_string() },
    ];

    let mut result_literals = literals.to_empty();
    
    alternate_literals(&es, &mut result_literals, |e, lits3| {
        if lits3.limit_size() == 3 {
            lits3.content.push(format!("{}-suffix", e.value));
        }
    });

    assert!(!result_literals.is_empty());
    assert!(result_literals.cross_product(&unioned_literals));
}

#[test]
#[should_panic]
fn test_alternate_literals_empty_union() {
    let literals = Literals::new(10);
    let mut result_literals = literals.to_empty();

    let es = vec![
        Hir { value: "h1".to_string() },
    ];

    alternate_literals(&es, &mut result_literals, |e, lits3| {
        // This will lead to an empty result if executed
        lits3.set_limit_size(1); // minimal limit to guarantee failure
        lits3.content.push(format!("{}-suffix", e.value));
    });
}

