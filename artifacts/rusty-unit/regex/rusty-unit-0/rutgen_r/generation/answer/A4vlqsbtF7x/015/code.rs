// Answer 0

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Literal {
    cut: bool,
    value: String,
}

impl Literal {
    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    fn len(&self) -> usize {
        self.value.len()
    }

    fn clear(&mut self) {
        self.value.clear();
    }

    fn cut(&mut self) {
        self.cut = true;
    }

    fn truncate(&mut self, len: usize) {
        self.value.truncate(len);
    }
}

struct Literals {
    lits: Vec<Literal>,
}

impl Literals {
    fn to_empty() -> Literals {
        Literals { lits: vec![] }
    }
}

fn position(lit1: &Literal, lit2: &Literal) -> Option<usize> {
    lit2.value.find(&lit1.value)
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
fn test_unambiguous_prefixes() {
    let literals = Literals {
        lits: vec![
            Literal { cut: false, value: "abc".to_string() },
            Literal { cut: false, value: "ab".to_string() },
            Literal { cut: false, value: "abcd".to_string() },
            Literal { cut: false, value: "a".to_string() },
        ],
    };
    
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 2);
    assert!(result.lits.contains(&Literal { cut: false, value: "abc".to_string() }));
    assert!(result.lits.contains(&Literal { cut: false, value: "a".to_string() }));
}

#[test]
fn test_unambiguous_prefixes_no_empty_literals() {
    let literals = Literals {
        lits: vec![
            Literal { cut: false, value: "xyz".to_string() },
            Literal { cut: false, value: "xy".to_string() },
            Literal { cut: false, value: "x".to_string() },
        ],
    };
    
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 2);
    assert!(result.lits.contains(&Literal { cut: false, value: "xyz".to_string() }));
    assert!(result.lits.contains(&Literal { cut: false, value: "x".to_string() }));
}

#[test]
fn test_unambiguous_prefixes_with_same_length_literals() {
    let literals = Literals {
        lits: vec![
            Literal { cut: false, value: "a".to_string() },
            Literal { cut: false, value: "b".to_string() },
            Literal { cut: false, value: "c".to_string() },
        ],
    };
    
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 3);
    assert!(result.lits.contains(&Literal { cut: false, value: "a".to_string() }));
    assert!(result.lits.contains(&Literal { cut: false, value: "b".to_string() }));
    assert!(result.lits.contains(&Literal { cut: false, value: "c".to_string() }));
}

