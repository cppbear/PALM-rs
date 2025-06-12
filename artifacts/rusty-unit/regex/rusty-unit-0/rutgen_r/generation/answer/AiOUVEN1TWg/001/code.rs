// Answer 0

#[test]
fn test_is_start_true() {
    struct Input {
        pos: usize,
    }

    impl Input {
        pub fn is_start(&self) -> bool {
            self.pos == 0
        }
    }

    let input = Input { pos: 0 };
    assert!(input.is_start());
}

#[test]
fn test_is_start_false() {
    struct Input {
        pos: usize,
    }

    impl Input {
        pub fn is_start(&self) -> bool {
            self.pos == 0
        }
    }

    let input = Input { pos: 1 };
    assert!(!input.is_start());
}

#[test]
fn test_is_start_boundary() {
    struct Input {
        pos: usize,
    }

    impl Input {
        pub fn is_start(&self) -> bool {
            self.pos == 0
        }
    }

    let input = Input { pos: usize::MAX };
    assert!(!input.is_start());
}

