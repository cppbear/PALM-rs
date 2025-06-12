// Answer 0

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Literal {
    value: String,
    cut: bool,
}

struct Literals {
    lits: Vec<Literal>,
}

impl Literals {
    fn to_empty(&self) -> Literals {
        Literals { lits: Vec::new() }
    }
}

impl Literal {
    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    
    fn truncate(&mut self, len: usize) {
        self.value.truncate(len);
    }

    fn clear(&mut self) {
        self.value.clear();
    }

    fn cut(&mut self) {
        self.cut = true;
    }
}

fn position(a: &Literal, b: &Literal) -> Option<usize> {
    if b.value.starts_with(&a.value) {
        Some(a.value.len())
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
                if candidate.value.len() < lit2.value.len() {
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
fn test_unambiguous_prefixes_basic() {
    let literal1 = Literal { value: "abc".to_string(), cut: false };
    let literal2 = Literal { value: "ab".to_string(), cut: false };
    let literal3 = Literal { value: "a".to_string(), cut: false };
    let literals = Literals { lits: vec![literal1, literal2, literal3] };

    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 2);
    assert_eq!(result.lits[0].value, "abc");
    assert_eq!(result.lits[1].value, "ab");
}

#[test]
fn test_unambiguous_prefixes_contain_cut_literals() {
    let literal1 = Literal { value: "abc".to_string(), cut: false };
    let literal2 = Literal { value: "ab".to_string(), cut: true };
    let literals = Literals { lits: vec![literal1, literal2] };

    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 1);
    assert_eq!(result.lits[0].value, "abc");
    assert!(result.lits[0].cut);
}

#[test]
fn test_unambiguous_prefixes_empty_input() {
    let literals = Literals { lits: Vec::new() };

    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 0);
}

#[test]
fn test_unambiguous_prefixes_multiple_similar() {
    let literal1 = Literal { value: "a".to_string(), cut: false };
    let literal2 = Literal { value: "aa".to_string(), cut: false };
    let literal3 = Literal { value: "aaa".to_string(), cut: true };
    let literals = Literals { lits: vec![literal1, literal2, literal3] };

    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 1);
    assert_eq!(result.lits[0].value, "aa");
}

