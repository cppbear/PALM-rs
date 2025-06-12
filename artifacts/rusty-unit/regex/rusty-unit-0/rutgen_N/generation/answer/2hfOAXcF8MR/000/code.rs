// Answer 0

#[test]
fn test_deref() {
    struct TestStruct {
        insts: Box<i32>,
    }

    impl TestStruct {
        fn new(value: i32) -> Self {
            TestStruct {
                insts: Box::new(value),
            }
        }

        fn deref(&self) -> &i32 {
            &*self.insts
        }
    }

    let test_value = 42;
    let test_instance = TestStruct::new(test_value);
    assert_eq!(*test_instance.deref(), test_value);
}

