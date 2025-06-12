// Answer 0

#[derive(Clone, PartialEq, Eq, Debug)]
struct Literal {
    value: String,
    cut: bool,
}

impl Literal {
    fn new(value: &str) -> Self {
        Literal {
            value: value.to_string(),
            cut: false,
        }
    }

    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    fn len(&self) -> usize {
        self.value.len()
    }

    fn clear(&mut self) {
        self.value.clear();
    }

    fn truncate(&mut self, len: usize) {
        if len < self.len() {
            self.value.truncate(len);
        }
    }

    fn cut(&mut self) {
        self.cut = true;
    }
}

#[derive(Debug)]
struct Literals {
    lits: Vec<Literal>,
}

impl Literals {
    fn new() -> Self {
        Literals { lits: Vec::new() }
    }

    fn to_empty(&self) -> Self {
        Literals::new()
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
                if candidate.len() < lit2.len() {
                    if let Some(i) = position(&candidate, &lit2) {
                        candidate.cut();
                        let mut lit3 = lit2.clone();
                        lit3.truncate(i);
                        lit3.cut();
                        old.push(lit3);
                        lit2.clear();
                    }
                } else {
                    if let Some(i) = position(&lit2, &candidate) {
                        lit2.cut();
                        let mut new_candidate = candidate.clone();
                        new_candidate.truncate(i);
                        new_candidate.cut();
                        old.push(new_candidate);
                        candidate.clear();
                    }
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

fn position(lit1: &Literal, lit2: &Literal) -> Option<usize> {
    lit2.value.find(&lit1.value)
}

#[test]
fn test_unambiguous_prefixes_with_literals() {
    let mut literals = Literals::new();
    literals.lits.push(Literal::new("abc"));
    literals.lits.push(Literal::new("abcd"));
    literals.lits.push(Literal::new("ab"));
    literals.lits.push(Literal::new("a"));

    let result = literals.unambiguous_prefixes();

    assert_eq!(result.lits.len(), 2);
    assert_eq!(result.lits[0], Literal::new("a"));
    assert_eq!(result.lits[1], Literal::new("abc"));
}

#[test]
fn test_unambiguous_prefixes_with_non_overlapping_literals() {
    let mut literals = Literals::new();
    literals.lits.push(Literal::new("xyz"));
    literals.lits.push(Literal::new("uvw"));
    literals.lits.push(Literal::new("mnop"));

    let result = literals.unambiguous_prefixes();

    assert_eq!(result.lits.len(), 3);
    assert_eq!(result.lits[0], Literal::new("mnop"));
    assert_eq!(result.lits[1], Literal::new("uvw"));
    assert_eq!(result.lits[2], Literal::new("xyz"));
}

#[test]
fn test_unambiguous_prefixes_with_empty_literal() {
    let mut literals = Literals::new();
    literals.lits.push(Literal::new("test"));
    literals.lits.push(Literal::new(""));

    let result = literals.unambiguous_prefixes();

    assert_eq!(result.lits.len(), 1);
    assert_eq!(result.lits[0], Literal::new("test"));
}

