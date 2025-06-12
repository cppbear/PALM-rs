// Answer 0

struct DepthChecker {
    depth: usize,
}

impl DepthChecker {
    fn new(depth: usize) -> Self {
        DepthChecker { depth }
    }

    fn decrement_depth(&mut self) {
        self.depth = self.depth.checked_sub(1).unwrap();
    }
}

#[test]
#[should_panic]
fn test_decrement_depth_panics_on_zero() {
    let mut checker = DepthChecker::new(0);
    checker.decrement_depth();
}

#[test]
fn test_decrement_depth_happy_path() {
    let mut checker = DepthChecker::new(1);
    checker.decrement_depth();
    assert_eq!(checker.depth, 0);
}

#[test]
fn test_decrement_depth_multiple_calls() {
    let mut checker = DepthChecker::new(5);
    for _ in 0..5 {
        checker.decrement_depth();
    }
    assert_eq!(checker.depth, 0);
}

#[test]
#[should_panic]
fn test_decrement_depth_panics_when_already_zero() {
    let mut checker = DepthChecker::new(0);
    checker.decrement_depth(); // First call should panic
    checker.decrement_depth(); // This call should not be reached
}

