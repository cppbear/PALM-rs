// Answer 0

#[derive(Debug)]
struct TestStruct(u8);

impl TestStruct {
    fn set_match(&mut self) {
        self.0 |= 0b0000000_1;
    }
}

#[test]
fn test_set_match_initial_zero() {
    let mut test_instance = TestStruct(0);
    test_instance.set_match();
    assert_eq!(test_instance.0, 1);
}

#[test]
fn test_set_match_initial_one() {
    let mut test_instance = TestStruct(1);
    test_instance.set_match();
    assert_eq!(test_instance.0, 1);
}

#[test]
fn test_set_match_initial_two() {
    let mut test_instance = TestStruct(2);
    test_instance.set_match();
    assert_eq!(test_instance.0, 3);
}

#[test]
fn test_set_match_initial_three() {
    let mut test_instance = TestStruct(3);
    test_instance.set_match();
    assert_eq!(test_instance.0, 3);
}

#[test]
fn test_set_match_initial_five() {
    let mut test_instance = TestStruct(5);
    test_instance.set_match();
    assert_eq!(test_instance.0, 5);
}

#[test]
fn test_set_match_initial_max() {
    let mut test_instance = TestStruct(255);
    test_instance.set_match();
    assert_eq!(test_instance.0, 255);
}

