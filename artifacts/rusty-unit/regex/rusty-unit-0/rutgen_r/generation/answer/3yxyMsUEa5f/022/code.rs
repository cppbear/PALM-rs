// Answer 0

#[test]
fn test_cross_product_successful() {
    struct Literal {
        len: usize,
        cut: bool,
    }

    impl Literal {
        fn new(len: usize, cut: bool) -> Self {
            Self { len, cut }
        }

        fn is_cut(&self) -> bool {
            self.cut
        }

        fn len(&self) -> usize {
            self.len
        }

        fn extend(&mut self, other: &Literal) {
            self.len += other.len;
        }
    }

    struct Literals {
        literals: Vec<Literal>,
    }

    impl Literals {
        fn new(literals: Vec<Literal>) -> Self {
            Self { literals }
        }

        fn is_empty(&self) -> bool {
            self.literals.is_empty()
        }

        fn literals(&self) -> &Vec<Literal> {
            &self.literals
        }
    }

    struct LiteralSet {
        lits: Vec<Literal>,
        limit_size: usize,
    }

    impl LiteralSet {
        fn new(limit_size: usize, lits: Vec<Literal>) -> Self {
            Self { lits, limit_size }
        }

        fn is_empty(&self) -> bool {
            self.lits.is_empty()
        }

        fn any_complete(&self) -> bool {
            !self.is_empty()
        }

        fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.len()).sum()
        }

        fn remove_complete(&mut self) -> Vec<Literal> {
            let complete = self.lits.clone();
            self.lits.clear();
            complete
        }

        fn cross_product(&mut self, lits: &Literals) -> bool {
            if lits.is_empty() {
                return true;
            }
            let mut size_after;
            if self.is_empty() || !self.any_complete() {
                size_after = self.num_bytes();
                for lits_lit in lits.literals() {
                    size_after += lits_lit.len();
                }
            } else {
                size_after = self.lits.iter().fold(0, |accum, lit| {
                    accum + if lit.is_cut() { lit.len() } else { 0 }
                });
                for lits_lit in lits.literals() {
                    for self_lit in &self.lits {
                        if !self_lit.is_cut() {
                            size_after += self_lit.len() + lits_lit.len();
                        }
                    }
                }
            }
            if size_after > self.limit_size {
                return false;
            }

            let mut base = self.remove_complete();
            if base.is_empty() {
                base = vec![Literal::new(0, false)];
            }
            for lits_lit in lits.literals() {
                for mut self_lit in base.clone() {
                    self_lit.extend(&**lits_lit);
                    self_lit.cut = lits_lit.cut;
                    self.lits.push(self_lit);
                }
            }
            true
        }
    }

    let mut set = LiteralSet::new(10, vec![Literal::new(1, true), Literal::new(1, true)]);
    let lits = Literals::new(vec![Literal::new(1, false), Literal::new(1, false)]);

    assert_eq!(set.cross_product(&lits), true);
}

