// Answer 0

#[derive(Debug)]
struct TestStruct {
    end: u8,
}

impl TestStruct {
    fn upper(&self) -> u8 {
        self.end
    }
}

#[test]
fn test_upper() {
    let test_instance = TestStruct { end: 5 };
    assert_eq!(test_instance.upper(), 5);
}

#[test]
fn test_upper_zero() {
    let test_instance = TestStruct { end: 0 };
    assert_eq!(test_instance.upper(), 0);
}

#[test]
fn test_upper_max() {
    let test_instance = TestStruct { end: u8::MAX };
    assert_eq!(test_instance.upper(), u8::MAX);
}

