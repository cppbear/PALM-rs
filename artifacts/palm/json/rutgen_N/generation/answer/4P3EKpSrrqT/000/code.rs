// Answer 0

#[derive(Default)]
struct TestStruct;

impl TestStruct {
    fn set_failed(&mut self, failed: &mut bool) {
        *failed = true;
    }
}

#[test]
fn test_set_failed() {
    let mut failed = false;
    let mut test_struct = TestStruct::default();
    test_struct.set_failed(&mut failed);
    assert!(failed);
}

#[test]
fn test_set_failed_already_failed() {
    let mut failed = true;
    let mut test_struct = TestStruct::default();
    test_struct.set_failed(&mut failed);
    assert!(failed);
}

