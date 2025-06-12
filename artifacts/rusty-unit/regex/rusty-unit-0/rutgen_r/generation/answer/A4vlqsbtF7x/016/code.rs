// Answer 0

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct Literal {
    value: String,
    cut: bool,
}

impl Literal {
    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    fn len(&self) -> usize {
        self.value.len()
    }

    fn cut(&mut self) {
        self.cut = true;
    }

    fn clear(&mut self) {
        self.value.clear();
    }

    fn truncate(&mut self, len: usize) {
        self.value.truncate(len);
    }
}

#[derive(Clone, Default)]
struct Literals {
    lits: Vec<Literal>,
}

impl Literals {
    fn to_empty(&self) -> Literals {
        Literals::default()
    }
}

fn position(a: &Literal, b: &Literal) -> Option<usize> {
    if b.value.starts_with(&a.value) {
        Some(a.len())
    } else {
        None
    }
}

impl Literals {
    pub fn unambiguous_prefixes(&self) -> Literals {
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

#[test]
fn test_unambiguous_prefixes_single_literal() {
    let mut literals = Literals {
        lits: vec![Literal { value: "abc".to_string(), cut: false }],
    };
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 1);
    assert_eq!(result.lits[0].value, "abc");
}

#[test]
fn test_unambiguous_prefixes_multiple_literals_no_overlap() {
    let mut literals = Literals {
        lits: vec![
            Literal { value: "abc".to_string(), cut: false },
            Literal { value: "def".to_string(), cut: false },
        ],
    };
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 2);
}

#[test]
fn test_unambiguous_prefixes_multiple_literals_with_overlap() {
    let mut literals = Literals {
        lits: vec![
            Literal { value: "ab".to_string(), cut: false },
            Literal { value: "abc".to_string(), cut: false },
            Literal { value: "abcd".to_string(), cut: false },
        ],
    };
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 1);
    assert_eq!(result.lits[0].value, "ab");
}

#[test]
fn test_unambiguous_prefixes_empty_lit_in_input() {
    let mut literals = Literals {
        lits: vec![
            Literal { value: "".to_string(), cut: false },
            Literal { value: "abc".to_string(), cut: false },
        ],
    };
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 1);
    assert_eq!(result.lits[0].value, "abc");
}

#[test]
fn test_unambiguous_prefixes_identical_literals() {
    let mut literals = Literals {
        lits: vec![
            Literal { value: "abc".to_string(), cut: false },
            Literal { value: "abc".to_string(), cut: false },
        ],
    };
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 1);
    assert_eq!(result.lits[0].value, "abc");
}

