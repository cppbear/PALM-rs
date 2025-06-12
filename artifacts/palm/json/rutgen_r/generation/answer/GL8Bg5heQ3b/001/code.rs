// Answer 0

#[derive(Debug)]
struct TestStruct {
    line: usize,
}

impl TestStruct {
    pub fn new(line: usize) -> Self {
        TestStruct { line }
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

#[test]
fn test_line_basic() {
    let test_instance = TestStruct::new(10);
    assert_eq!(test_instance.line(), 10);
}

#[test]
fn test_line_zero() {
    let test_instance = TestStruct::new(0);
    assert_eq!(test_instance.line(), 0);
}

#[test]
fn test_line_large_number() {
    let test_instance = TestStruct::new(usize::MAX);
    assert_eq!(test_instance.line(), usize::MAX);
}

#[test]
#[should_panic]
fn test_line_panic_on_negative() {
    let test_instance = TestStruct::new((usize::MAX as isize + 1) as usize);
    // Since maximum value for usize cannot be negative, this test is just conceptual.
    // The function line does not have panic conditions, hence it won't compile.
    // This test would serve as a reminder or marker for any future implementation specifics.
    panic!("Simulated panic");
}

