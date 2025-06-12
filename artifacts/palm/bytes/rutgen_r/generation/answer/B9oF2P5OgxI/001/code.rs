// Answer 0

#[test]
fn test_remaining_zero() {
    struct BufA {
        len: usize,
    }
    
    struct BufB {
        len: usize,
    }

    impl BufA {
        fn remaining(&self) -> usize {
            self.len
        }
    }

    impl BufB {
        fn remaining(&self) -> usize {
            self.len
        }
    }

    let a = BufA { len: 0 };
    let b = BufB { len: 0 };

    assert_eq!(a.remaining().saturating_add(b.remaining()), 0);
}

#[test]
fn test_remaining_non_zero() {
    struct BufA {
        len: usize,
    }
    
    struct BufB {
        len: usize,
    }

    impl BufA {
        fn remaining(&self) -> usize {
            self.len
        }
    }

    impl BufB {
        fn remaining(&self) -> usize {
            self.len
        }
    }

    let a = BufA { len: 5 };
    let b = BufB { len: 10 };

    assert_eq!(a.remaining().saturating_add(b.remaining()), 15);
}

#[test]
fn test_remaining_saturation() {
    struct BufA {
        len: usize,
    }
    
    struct BufB {
        len: usize,
    }

    impl BufA {
        fn remaining(&self) -> usize {
            self.len
        }
    }

    impl BufB {
        fn remaining(&self) -> usize {
            self.len
        }
    }

    let a = BufA { len: usize::MAX };
    let b = BufB { len: 1 };

    assert_eq!(a.remaining().saturating_add(b.remaining()), usize::MAX);
}

#[test]
fn test_remaining_large_numbers() {
    struct BufA {
        len: usize,
    }
    
    struct BufB {
        len: usize,
    }

    impl BufA {
        fn remaining(&self) -> usize {
            self.len
        }
    }

    impl BufB {
        fn remaining(&self) -> usize {
            self.len
        }
    }

    let a = BufA { len: usize::MAX };
    let b = BufB { len: usize::MAX };

    assert_eq!(a.remaining().saturating_add(b.remaining()), usize::MAX);
}

