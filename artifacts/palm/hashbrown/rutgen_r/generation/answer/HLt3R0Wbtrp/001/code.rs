// Answer 0

#[test]
fn test_deref_mut() {
    struct TestStruct {
        value: i32,
    }

    impl TestStruct {
        fn new(value: i32) -> Self {
            TestStruct { value }
        }

        fn deref_mut(&mut self) -> &mut i32 {
            &mut self.value
        }
    }

    let mut test_instance = TestStruct::new(10);
    let value_ref: &mut i32 = test_instance.deref_mut();
    *value_ref += 5;

    assert_eq!(*value_ref, 15);
}

#[test]
fn test_deref_mut_boundary() {
    struct TestStruct {
        value: i32,
    }

    impl TestStruct {
        fn new(value: i32) -> Self {
            TestStruct { value }
        }

        fn deref_mut(&mut self) -> &mut i32 {
            &mut self.value
        }
    }

    let mut test_instance = TestStruct::new(i32::MIN);
    let value_ref: &mut i32 = test_instance.deref_mut();
    *value_ref += 1;

    assert_eq!(*value_ref, i32::MIN + 1);
}

