// Answer 0

#[test]
fn test_unambiguous_prefixes_with_non_empty_literals() {
    use std::cmp::Ord;

    #[derive(Clone, Eq, PartialEq, Ord)]
    struct TestLiteral {
        v: Vec<u8>,
        cut: bool,
    }

    impl TestLiteral {
        fn new(v: Vec<u8>) -> TestLiteral {
            TestLiteral { v, cut: false }
        }
        fn is_empty(&self) -> bool {
            self.v.is_empty()
        }
        fn len(&self) -> usize {
            self.v.len()
        }
        fn cut(&mut self) {
            self.cut = true;
        }
        fn clear(&mut self) {
            self.v.clear();
        }
        fn truncate(&mut self, len: usize) {
            self.v.truncate(len);
        }
    }

    #[derive(Clone)]
    struct TestLiterals {
        lits: Vec<TestLiteral>,
    }

    impl TestLiterals {
        fn new(lits: Vec<TestLiteral>) -> TestLiterals {
            TestLiterals { lits }
        }
        fn to_empty(&self) -> TestLiterals {
            TestLiterals::new(vec![])
        }
        fn unambiguous_prefixes(&self) -> TestLiterals {
            if self.lits.is_empty() {
                return self.to_empty();
            }

            let mut old: Vec<TestLiteral> = self.lits.iter().cloned().collect();
            let mut new = self.to_empty();

            'OUTER: while let Some(mut candidate) = old.pop() {
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
                    if candidate == *lit2 {
                        candidate.cut = candidate.cut || lit2.cut;
                        lit2.cut = candidate.cut;
                        continue 'OUTER;
                    }
                    if candidate.len() < lit2.len() {
                        // Assume position function is defined elsewhere
                        if let Some(i) = position(&candidate.v, &lit2.v) {
                            candidate.cut();
                            let mut lit3 = lit2.clone();
                            lit3.truncate(i);
                            lit3.cut();
                            old.push(lit3);
                            lit2.clear();
                        }
                    } else {
                        if let Some(i) = position(&lit2.v, &candidate.v) {
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

    let lit1 = TestLiteral::new(vec![1]); // Not empty
    let lit2 = TestLiteral::new(vec![1, 2]); // Supposed to be unambiguous with lit1
    let lit3 = TestLiteral::new(vec![2]); // Ambiguous with lit2

    let test_literals = TestLiterals::new(vec![lit1.clone(), lit2.clone(), lit3.clone()]);

    let result = test_literals.unambiguous_prefixes();

    assert_eq!(result.lits.len(), 2); // Expecting lit1 and lit2 to be in the result
    assert!(result.lits.contains(&lit1));
    assert!(result.lits.contains(&lit2));
}

