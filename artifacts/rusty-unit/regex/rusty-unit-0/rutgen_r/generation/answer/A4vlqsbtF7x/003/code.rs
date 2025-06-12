// Answer 0

#[derive(Clone, Debug, PartialEq, Eq)]
struct Literal {
    cut: bool,
    content: String,
}

impl Literal {
    fn new(content: &str) -> Self {
        Self {
            cut: false,
            content: content.to_string(),
        }
    }

    fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    fn clear(&mut self) {
        self.content.clear();
    }

    fn truncate(&mut self, len: usize) {
        self.content.truncate(len);
    }

    fn cut(&mut self) {
        self.cut = true;
    }
}

#[derive(Clone, Debug)]
struct Literals {
    lits: Vec<Literal>,
}

impl Literals {
    fn new() -> Self {
        Self {
            lits: Vec::new(),
        }
    }

    fn to_empty(&self) -> Literals {
        Literals::new()
    }

    fn add_literal(&mut self, lit: Literal) {
        self.lits.push(lit);
    }

    fn unambiguous_prefixes(&self) -> Literals {
        if self.lits.is_empty() {
            return self.to_empty();
        }
        let mut old: Vec<Literal> = self.lits.iter().cloned().collect();
        let mut new = self.to_empty();
    'OUTER:
        while let Some(mut candidate) = old.pop() {
            if candidate.is_empty() {
                continue;
            }
            if new.lits.is_empty() {
                new.lits.push(candidate);
                continue;
            }
            for lit2 in &mut new.lits {
                if lit2.is_empty() {
                    continue;
                }
                if &candidate == lit2 {
                    candidate.cut = candidate.cut || lit2.cut;
                    lit2.cut = candidate.cut;
                    continue 'OUTER;
                }
                if candidate.content.len() < lit2.content.len() {
                    candidate.cut();
                    old.push(lit2.clone());
                    lit2.clear();
                } else {
                    lit2.cut();
                    old.push(candidate.clone());
                    candidate.clear();
                }
                if candidate.is_empty() {
                    continue 'OUTER;
                }
            }
            new.lits.push(candidate);
        }
        new.lits.retain(|lit| !lit.is_empty());
        new.lits.sort();
        new.lits.dedup();
        new
    }
}

#[test]
fn test_unambiguous_prefixes_non_empty_literals() {
    let mut literals = Literals::new();
    literals.add_literal(Literal::new("abc"));
    literals.add_literal(Literal::new("abcd"));
    literals.add_literal(Literal::new("ab"));
    literals.add_literal(Literal::new("a"));

    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 2);
    assert_eq!(result.lits[0], Literal::new("a"));
    assert_eq!(result.lits[1], Literal::new("abc"));
}

#[test]
fn test_unambiguous_prefixes_duplicate_literals() {
    let mut literals = Literals::new();
    literals.add_literal(Literal::new("abc"));
    literals.add_literal(Literal::new("abc")); // Duplicate

    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 1);
    assert_eq!(result.lits[0], Literal::new("abc"));
}

#[test]
fn test_unambiguous_prefixes_empty_candidate() {
    let mut literals = Literals::new();
    literals.add_literal(Literal::new("abc"));
    literals.add_literal(Literal::new("")); // Empty candidate

    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 1);
    assert_eq!(result.lits[0], Literal::new("abc"));
}

#[test]
fn test_unambiguous_prefixes_all_empty_literals() {
    let mut literals = Literals::new();
    literals.add_literal(Literal::new(""));
    literals.add_literal(Literal::new(""));

    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 0);
}

