// Answer 0

#[test]
fn test_repeat_zero_or_more_literals_case_one() {
    struct Hir;
    struct Literals {
        is_empty: bool,
        limit_size: usize,
        cross_product_result: bool,
    }

    impl Literals {
        fn clone(&self) -> Self {
            Literals {
                is_empty: self.is_empty,
                limit_size: self.limit_size,
                cross_product_result: self.cross_product_result,
            }
        }

        fn to_empty(&self) -> Self {
            Literals {
                is_empty: true,
                limit_size: 0,
                cross_product_result: false,
            }
        }

        fn set_limit_size(&mut self, size: usize) {
            self.limit_size = size;
        }

        fn limit_size(&self) -> usize {
            self.limit_size
        }

        fn is_empty(&self) -> bool {
            self.is_empty
        }

        fn cross_product(&self, _other: &Self) -> bool {
            self.cross_product_result
        }

        fn cut(&mut self) {
            self.is_empty = true;
        }

        fn union(&mut self, _other: Literals) -> bool {
            false
        }
    }

    let mut lits = Literals {
        is_empty: false,
        limit_size: 10,
        cross_product_result: false,
    };

    let mock_hir = Hir;

    repeat_zero_or_more_literals(&mock_hir, &mut lits, |_, _| {});

    assert!(lits.is_empty);
}

#[test]
fn test_repeat_zero_or_more_literals_case_two() {
    struct Hir;
    struct Literals {
        is_empty: bool,
        limit_size: usize,
        cross_product_result: bool,
    }

    impl Literals {
        fn clone(&self) -> Self {
            Literals {
                is_empty: self.is_empty,
                limit_size: self.limit_size,
                cross_product_result: self.cross_product_result,
            }
        }

        fn to_empty(&self) -> Self {
            Literals {
                is_empty: true,
                limit_size: 0,
                cross_product_result: false,
            }
        }

        fn set_limit_size(&mut self, size: usize) {
            self.limit_size = size;
        }

        fn limit_size(&self) -> usize {
            self.limit_size
        }

        fn is_empty(&self) -> bool {
            self.is_empty
        }

        fn cross_product(&self, _other: &Self) -> bool {
            // Simulate true for the opposite condition
            !self.cross_product_result
        }

        fn cut(&mut self) {
            self.is_empty = true;
        }

        fn union(&mut self, _other: Literals) -> bool {
            false
        }
    }

    let mut lits = Literals {
        is_empty: false,
        limit_size: 10,
        cross_product_result: false,
    };

    let mock_hir = Hir;

    repeat_zero_or_more_literals(&mock_hir, &mut lits, |_, _| {});

    assert!(lits.is_empty);
}

