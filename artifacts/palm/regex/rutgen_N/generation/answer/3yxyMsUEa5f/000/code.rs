// Answer 0

#[test]
fn test_cross_product_with_empty_literals() {
    struct Literals {
        literals: Vec<Literal>,
    }
    
    impl Literals {
        fn is_empty(&self) -> bool {
            self.literals.is_empty()
        }
        
        fn literals(&self) -> &Vec<Literal> {
            &self.literals
        }
    }

    struct Literal {
        content: String,
        cut: bool,
    }

    impl Literal {
        fn len(&self) -> usize {
            self.content.len()
        }
        
        fn cut(&self) -> bool {
            self.cut
        }
        
        fn extend(&mut self, other: &str) {
            self.content.push_str(other);
        }

        fn empty() -> Self {
            Literal { content: String::new(), cut: false }
        }
    }

    struct MySet {
        lits: Vec<Literal>,
        limit_size: usize,
    }

    impl MySet {
        fn is_empty(&self) -> bool {
            self.lits.is_empty()
        }

        fn any_complete(&self) -> bool {
            self.lits.iter().any(|lit| lit.cut)
        }

        fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.len()).sum()
        }

        fn literals(&self) -> &Vec<Literal> {
            &self.lits
        }
        
        fn remove_complete(&mut self) -> Vec<Literal> {
            self.lits.drain_filter(|lit| lit.cut).collect()
        }

        pub fn cross_product(&mut self, lits: &Literals) -> bool {
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
                    accum + if lit.cut() { lit.len() } else { 0 }
                });
                for lits_lit in lits.literals() {
                    for self_lit in self.literals() {
                        if !self_lit.cut() {
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
                base = vec![Literal::empty()];
            }
            for lits_lit in lits.literals() {
                for mut self_lit in base.clone() {
                    self_lit.extend(&**lits_lit);
                    self_lit.cut = lits_lit.cut();
                    self.lits.push(self_lit);
                }
            }
            true
        }
    }

    let mut my_set = MySet { lits: vec![], limit_size: 10 };
    let literals = Literals { literals: vec![] };
    assert!(my_set.cross_product(&literals));
}

#[test]
fn test_cross_product_exceeding_limit() {
    struct Literals {
        literals: Vec<Literal>,
    }
    
    impl Literals {
        fn is_empty(&self) -> bool {
            self.literals.is_empty()
        }
        
        fn literals(&self) -> &Vec<Literal> {
            &self.literals
        }
    }

    struct Literal {
        content: String,
        cut: bool,
    }

    impl Literal {
        fn len(&self) -> usize {
            self.content.len()
        }
        
        fn cut(&self) -> bool {
            self.cut
        }
        
        fn extend(&mut self, other: &str) {
            self.content.push_str(other);
        }

        fn empty() -> Self {
            Literal { content: String::new(), cut: false }
        }
    }

    struct MySet {
        lits: Vec<Literal>,
        limit_size: usize,
    }

    impl MySet {
        fn is_empty(&self) -> bool {
            self.lits.is_empty()
        }

        fn any_complete(&self) -> bool {
            self.lits.iter().any(|lit| lit.cut)
        }

        fn num_bytes(&self) -> usize {
            self.lits.iter().map(|lit| lit.len()).sum()
        }

        fn literals(&self) -> &Vec<Literal> {
            &self.lits
        }
        
        fn remove_complete(&mut self) -> Vec<Literal> {
            self.lits.drain_filter(|lit| lit.cut).collect()
        }

        pub fn cross_product(&mut self, lits: &Literals) -> bool {
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
                    accum + if lit.cut() { lit.len() } else { 0 }
                });
                for lits_lit in lits.literals() {
                    for self_lit in self.literals() {
                        if !self_lit.cut() {
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
                base = vec![Literal::empty()];
            }
            for lits_lit in lits.literals() {
                for mut self_lit in base.clone() {
                    self_lit.extend(&**lits_lit);
                    self_lit.cut = lits_lit.cut();
                    self.lits.push(self_lit);
                }
            }
            true
        }
    }

    let mut my_set = MySet { lits: vec![Literal { content: "abc".to_string(), cut: false }], limit_size: 5 };
    let literals = Literals { literals: vec![Literal { content: "de".to_string(), cut: false }] };
    assert!(!my_set.cross_product(&literals));
}

