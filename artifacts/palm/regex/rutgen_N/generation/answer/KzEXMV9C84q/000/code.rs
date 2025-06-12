// Answer 0

#[test]
fn test_repeat_zero_or_more_literals_empty() {
    struct DummyHir;
    struct DummyLiterals {
        limit_size: usize,
        is_empty: bool,
    }

    impl DummyLiterals {
        fn new(limit_size: usize, is_empty: bool) -> Self {
            Self { limit_size, is_empty }
        }

        fn clone(&self) -> Self {
            Self {
                limit_size: self.limit_size,
                is_empty: self.is_empty,
            }
        }

        fn to_empty(&self) -> Self {
            Self::new(0, true)
        }

        fn set_limit_size(&mut self, _size: usize) {}

        fn limit_size(&self) -> usize {
            self.limit_size
        }

        fn is_empty(&self) -> bool {
            self.is_empty
        }

        fn cut(&mut self) {}

        fn cross_product(&self, _other: &Self) -> bool {
            !self.is_empty
        }

        fn add(&mut self, _literal: Literal) {}

        fn union(&mut self, _other: Self) -> bool {
            true
        }
    }
    
    struct Literal;

    let hir = DummyHir;
    let mut literals = DummyLiterals::new(10, true);

    repeat_zero_or_more_literals(&hir, &mut literals, |_, _| {});
    
    assert!(literals.is_empty());
}

#[test]
fn test_repeat_zero_or_more_literals_non_empty() {
    struct DummyHir;
    struct DummyLiterals {
        limit_size: usize,
        is_empty: bool,
    }

    impl DummyLiterals {
        fn new(limit_size: usize, is_empty: bool) -> Self {
            Self { limit_size, is_empty }
        }

        fn clone(&self) -> Self {
            Self {
                limit_size: self.limit_size,
                is_empty: self.is_empty,
            }
        }

        fn to_empty(&self) -> Self {
            Self::new(0, true)
        }

        fn set_limit_size(&mut self, _size: usize) {}

        fn limit_size(&self) -> usize {
            self.limit_size
        }

        fn is_empty(&self) -> bool {
            self.is_empty
        }

        fn cut(&mut self) {}

        fn cross_product(&self, _other: &Self) -> bool {
            !self.is_empty
        }

        fn add(&mut self, _literal: Literal) {}

        fn union(&mut self, _other: Self) -> bool {
            true
        }
    }
    
    struct Literal;

    let hir = DummyHir;
    let mut literals = DummyLiterals::new(10, false);

    repeat_zero_or_more_literals(&hir, &mut literals, |_, _| {});
    
    assert!(!literals.is_empty());
}

