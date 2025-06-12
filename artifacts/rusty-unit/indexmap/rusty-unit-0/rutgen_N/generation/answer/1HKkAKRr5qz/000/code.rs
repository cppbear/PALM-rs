// Answer 0

#[derive(Debug)]
struct TestStruct {
    value: i32,
}

impl TestStruct {
    fn new(value: i32) -> Self {
        TestStruct { value }
    }

    fn value_mut(&mut self) -> &mut i32 {
        &mut self.value
    }
}

#[test]
fn test_value_mut() {
    let mut test_struct = TestStruct::new(5);
    {
        let value_ref = test_struct.value_mut();
        *value_ref += 10;
    }
    assert_eq!(test_struct.value, 15);
}

#[test]
fn test_value_mut_boundary() {
    let mut test_struct = TestStruct::new(0);
    {
        let value_ref = test_struct.value_mut();
        *value_ref = -1;
    }
    assert_eq!(test_struct.value, -1);
}

