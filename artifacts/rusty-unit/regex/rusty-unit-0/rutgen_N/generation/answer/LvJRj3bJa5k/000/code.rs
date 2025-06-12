// Answer 0

#[derive(Clone)]
struct Literal {
    value: String,
}

impl Literal {
    fn len(&self) -> usize {
        self.value.len()
    }

    fn truncate(&mut self, new_len: usize) {
        self.value.truncate(new_len);
    }

    fn cut(&mut self) {
        // Cutting logic can be added if needed
    }
}

#[derive(Default)]
struct Literals {
    lits: Vec<Literal>,
}

impl Literals {
    fn min_len(&self) -> Option<usize> {
        self.lits.iter().map(|lit| lit.len()).min()
    }

    fn to_empty(&self) -> Literals {
        Literals::default()
    }
}

impl Literals {
    pub fn trim_suffix(&self, num_bytes: usize) -> Option<Literals> {
        if self.min_len().map(|len| len <= num_bytes).unwrap_or(true) {
            return None;
        }
        let mut new = self.to_empty();
        for mut lit in self.lits.iter().cloned() {
            let new_len = lit.len() - num_bytes;
            lit.truncate(new_len);
            lit.cut();
            new.lits.push(lit);
        }
        new.lits.sort();
        new.lits.dedup();
        Some(new)
    }
}

#[test]
fn test_trim_suffix_removes_valid_suffix() {
    let lit1 = Literal { value: "HelloWorld".to_string() };
    let lit2 = Literal { value: "GoodbyeWorld".to_string() };
    
    let mut literals = Literals::default();
    literals.lits.push(lit1);
    literals.lits.push(lit2);
    
    let trimmed = literals.trim_suffix(5).unwrap();

    assert_eq!(trimmed.lits.len(), 2);
    assert_eq!(trimmed.lits[0].value, "Hello");
    assert_eq!(trimmed.lits[1].value, "Goodbye");
}

#[test]
fn test_trim_suffix_removes_duplicates() {
    let lit1 = Literal { value: "HelloWorld".to_string() };
    let lit2 = Literal { value: "HelloWorld".to_string() };
    
    let mut literals = Literals::default();
    literals.lits.push(lit1);
    literals.lits.push(lit2);
    
    let trimmed = literals.trim_suffix(5).unwrap();

    assert_eq!(trimmed.lits.len(), 1);
    assert_eq!(trimmed.lits[0].value, "Hello");
}

#[test]
fn test_trim_suffix_returns_none_when_cutting_all() {
    let lit1 = Literal { value: "Hi".to_string() };
    
    let mut literals = Literals::default();
    literals.lits.push(lit1);

    let trimmed = literals.trim_suffix(2);
    
    assert!(trimmed.is_none());
}

#[test]
fn test_trim_suffix_returns_none_when_empty() {
    let literals = Literals::default();
    
    let trimmed = literals.trim_suffix(1);
    
    assert!(trimmed.is_none());
}

