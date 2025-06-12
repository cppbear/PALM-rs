// Answer 0

#[derive(Debug)]
struct TestStruct(u32);

impl TestStruct {
    fn is_eof(&self) -> bool {
        self.0 == 256
    }
}

#[test]
fn test_is_eof_true() {
    let instance = TestStruct(256);
    assert_eq!(instance.is_eof(), true);
}

#[test]
fn test_is_eof_false() {
    let instance = TestStruct(255);
    assert_eq!(instance.is_eof(), false);
}

#[test]
fn test_is_eof_boundary_high() {
    let instance = TestStruct(257);
    assert_eq!(instance.is_eof(), false);
}

#[test]
fn test_is_eof_boundary_low() {
    let instance = TestStruct(0);
    assert_eq!(instance.is_eof(), false);
}

