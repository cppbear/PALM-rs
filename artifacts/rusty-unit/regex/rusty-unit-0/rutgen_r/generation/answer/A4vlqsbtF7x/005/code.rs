// Answer 0

fn test_unambiguous_prefixes_non_empty() {
    struct Literal {
        cut: bool,
        content: String,
    }

    impl Clone for Literal {
        fn clone(&self) -> Self {
            Literal {
                cut: self.cut,
                content: self.content.clone(),
            }
        }
    }

    impl Literal {
        fn is_empty(&self) -> bool {
            self.content.is_empty()
        }
        
        fn len(&self) -> usize {
            self.content.len()
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

    struct Literals {
        lits: Vec<Literal>,
    }

    impl Literals {
        fn to_empty(&self) -> Literals {
            Literals { lits: Vec::new() }
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
                        if let Some(i) = position(&candidate.content, &lit2.content) {
                            candidate.cut();
                            let mut lit3 = lit2.clone();
                            lit3.truncate(i);
                            lit3.cut();
                            old.push(lit3);
                            lit2.clear();
                        }
                    } else {
                        if let Some(i) = position(&lit2.content, &candidate.content) {
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

    fn position(a: &str, b: &str) -> Option<usize> {
        if let Some(pos) = b.find(a) {
            Some(pos)
        } else {
            None
        }
    }

    let lit1 = Literal { cut: false, content: String::from("abc") };
    let lit2 = Literal { cut: false, content: String::from("abcd") };
    let lit3 = Literal { cut: false, content: String::from("ab") };
    let lit4 = Literal { cut: false, content: String::from("xyz") };

    let literals = Literals {
        lits: vec![lit1, lit2, lit3, lit4],
    };

    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 2);
}

fn test_unambiguous_prefixes_empty() {
    let literals = Literals { lits: Vec::new() };
    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 0);
}

fn test_unambiguous_prefixes_with_cut() {
    struct Literal {
        cut: bool,
        content: String,
    }

    impl Clone for Literal {
        fn clone(&self) -> Self {
            Literal {
                cut: self.cut,
                content: self.content.clone(),
            }
        }
    }
    
    // Other methods as above...

    let lit1 = Literal { cut: false, content: String::from("xy") };
    let lit2 = Literal { cut: false, content: String::from("x") };
    
    let literals = Literals {
        lits: vec![lit1, lit2],
    };

    let result = literals.unambiguous_prefixes();
    assert_eq!(result.lits.len(), 1);
}

