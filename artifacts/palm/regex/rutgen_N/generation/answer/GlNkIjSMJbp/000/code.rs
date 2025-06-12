// Answer 0

#[derive(Debug)]
struct TestStruct(usize);

impl TestStruct {
    fn is_eof(&self) -> bool {
        self.0 == 256
    }
}

#[test]
fn test_is_eof_true() {
    let test_struct = TestStruct(256);
    assert_eq!(test_struct.is_eof(), true);
}

#[test]
fn test_is_eof_false() {
    let test_struct = TestStruct(255);
    assert_eq!(test_struct.is_eof(), false);
}

#[test]
fn test_is_eof_zero() {
    let test_struct = TestStruct(0);
    assert_eq!(test_struct.is_eof(), false);
}

#[test]
fn test_is_eof_boundary() {
    let test_struct = TestStruct(257);
    assert_eq!(test_struct.is_eof(), false);
}

