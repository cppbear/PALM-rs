// Answer 0

#[test]
fn test_unambiguous_prefixes_non_empty() {
    #[derive(Clone, Eq, PartialEq)]
    struct MockLiteral {
        v: Vec<u8>,
        cut: bool,
    }

    #[derive(Clone, Eq, PartialEq)]
    struct MockLiterals {
        lits: Vec<MockLiteral>,
        limit_size: usize,
        limit_class: usize,
    }

    impl MockLiterals {
        fn empty() -> MockLiterals {
            MockLiterals {
                lits: Vec::new(),
                limit_size: 0,
                limit_class: 0,
            }
        }

        fn add(&mut self, lit: MockLiteral) {
            self.lits.push(lit);
        }

        fn unambiguous_prefixes(&self) -> MockLiterals {
            if self.lits.is_empty() {
                return Self::empty();
            }
            let mut old: Vec<MockLiteral> = self.lits.clone();
            let mut new = Self::empty();
            'OUTER:
            while let Some(mut candidate) = old.pop() {
                if candidate.v.is_empty() {
                    continue;
                }
                if new.lits.is_empty() {
                    new.lits.push(candidate);
                    continue;
                }
                for lit2 in &mut new.lits {
                    if lit2.v.is_empty() {
                        continue;
                    }
                    if candidate == *lit2 {
                        candidate.cut = candidate.cut || lit2.cut;
                        lit2.cut = candidate.cut;
                        continue 'OUTER;
                    }
                    if candidate.v.len() < lit2.v.len() {
                        if let Some(i) = position(&candidate.v, &lit2.v) {
                            candidate.cut();
                            let mut lit3 = lit2.clone();
                            lit3.v.truncate(i);
                            lit3.cut();
                            old.push(lit3);
                            lit2.v.clear();
                        }
                    } else {
                        if let Some(i) = position(&lit2.v, &candidate.v) {
                            lit2.cut();
                            let mut new_candidate = candidate.clone();
                            new_candidate.v.truncate(i);
                            new_candidate.cut();
                            old.push(new_candidate);
                            candidate.v.clear();
                        }
                    }
                    if candidate.v.is_empty() {
                        continue 'OUTER;
                    }
                }
                new.lits.push(candidate);
            }
            new.lits.retain(|lit| !lit.v.is_empty());
            new.lits.sort_by(|a, b| a.v.cmp(&b.v));
            new.lits.dedup();
            new
        }
    }

    let mut literals = MockLiterals::empty();
    literals.add(MockLiteral { v: vec![1, 2, 3], cut: false });
    literals.add(MockLiteral { v: vec![1, 2], cut: false });
    literals.add(MockLiteral { v: vec![4, 5, 6], cut: false });

    let result = literals.unambiguous_prefixes();
    
    assert_eq!(result.lits.len(), 2); // only the unambiguous ones should remain
    assert_eq!(result.lits[0].v, vec![4, 5, 6]); // should retain this
    assert_eq!(result.lits[1].v, vec![1, 2]); // should retain this
}

#[test]
fn test_unambiguous_prefixes_with_empty_literal() {
    #[derive(Clone, Eq, PartialEq)]
    struct MockLiteral {
        v: Vec<u8>,
        cut: bool,
    }

    #[derive(Clone, Eq, PartialEq)]
    struct MockLiterals {
        lits: Vec<MockLiteral>,
        limit_size: usize,
        limit_class: usize,
    }

    impl MockLiterals {
        fn empty() -> MockLiterals {
            MockLiterals {
                lits: Vec::new(),
                limit_size: 0,
                limit_class: 0,
            }
        }

        fn add(&mut self, lit: MockLiteral) {
            self.lits.push(lit);
        }

        fn unambiguous_prefixes(&self) -> MockLiterals {
            if self.lits.is_empty() {
                return Self::empty();
            }
            let mut old: Vec<MockLiteral> = self.lits.clone();
            let mut new = Self::empty();
            'OUTER:
            while let Some(mut candidate) = old.pop() {
                if candidate.v.is_empty() {
                    continue;
                }
                if new.lits.is_empty() {
                    new.lits.push(candidate);
                    continue;
                }
                for lit2 in &mut new.lits {
                    if lit2.v.is_empty() {
                        continue;
                    }
                    if candidate == *lit2 {
                        candidate.cut = candidate.cut || lit2.cut;
                        lit2.cut = candidate.cut;
                        continue 'OUTER;
                    }
                    if candidate.v.len() < lit2.v.len() {
                        if let Some(i) = position(&candidate.v, &lit2.v) {
                            candidate.cut();
                            let mut lit3 = lit2.clone();
                            lit3.v.truncate(i);
                            lit3.cut();
                            old.push(lit3);
                            lit2.v.clear();
                        }
                    } else {
                        if let Some(i) = position(&lit2.v, &candidate.v) {
                            lit2.cut();
                            let mut new_candidate = candidate.clone();
                            new_candidate.v.truncate(i);
                            new_candidate.cut();
                            old.push(new_candidate);
                            candidate.v.clear();
                        }
                    }
                    if candidate.v.is_empty() {
                        continue 'OUTER;
                    }
                }
                new.lits.push(candidate);
            }
            new.lits.retain(|lit| !lit.v.is_empty());
            new.lits.sort_by(|a, b| a.v.cmp(&b.v));
            new.lits.dedup();
            new
        }
    }

    let mut literals = MockLiterals::empty();
    literals.add(MockLiteral { v: vec![1, 2, 3], cut: false });
    literals.add(MockLiteral { v: vec![], cut: false }); // empty literal
    literals.add(MockLiteral { v: vec![1, 2, 4], cut: false });

    let result = literals.unambiguous_prefixes();

    assert_eq!(result.lits.len(), 2); // should filter out empty literal
    assert_eq!(result.lits[0].v, vec![1, 2, 4]); // should retain this
    assert_eq!(result.lits[1].v, vec![1, 2, 3]); // should retain this
}

