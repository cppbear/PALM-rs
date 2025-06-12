// Answer 0

#[derive(Debug)]
struct Uniform {
    start: i32,
    end: i32,
}

impl Uniform {
    fn new(start: i32, end: i32) -> Self {
        Uniform { start, end }
    }

    fn start(&self) -> i32 {
        self.start
    }

    fn end(&self) -> i32 {
        self.end
    }

    fn is_empty(&self) -> bool {
        !(self.start() <= self.end())
    }
}

#[test]
fn test_is_empty_with_empty_range() {
    let uniform = Uniform::new(5, 3);
    assert!(uniform.is_empty());
}

#[test]
fn test_is_empty_with_equal_start_and_end() {
    let uniform = Uniform::new(5, 5);
    assert!(uniform.is_empty());
}

#[test]
fn test_is_empty_with_valid_range() {
    let uniform = Uniform::new(3, 5);
    assert!(!uniform.is_empty());
}

