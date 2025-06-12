// Answer 0

#[derive(Debug)]
struct TestStruct {
    start: u8,
}

impl TestStruct {
    fn new(start: u8) -> Self {
        TestStruct { start }
    }

    fn lower(&self) -> u8 {
        self.start
    }
}

#[test]
fn test_lower() {
    let instance = TestStruct::new(5);
    assert_eq!(instance.lower(), 5);
}

#[test]
fn test_lower_zero() {
    let instance = TestStruct::new(0);
    assert_eq!(instance.lower(), 0);
}

#[test]
fn test_lower_max() {
    let instance = TestStruct::new(u8::MAX);
    assert_eq!(instance.lower(), u8::MAX);
}

