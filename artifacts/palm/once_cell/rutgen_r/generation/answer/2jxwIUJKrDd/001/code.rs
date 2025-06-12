// Answer 0

#[test]
fn test_fmt_with_initialized_cell() {
    struct TestStruct {
        cell: i32,
    }

    impl core::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TestStruct").field("cell", &self.cell).finish()
        }
    }

    let test_instance = TestStruct { cell: 42 };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", test_instance);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "TestStruct { cell: 42 }");
}

#[test]
fn test_fmt_with_zero_cell() {
    struct TestStruct {
        cell: i32,
    }

    impl core::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TestStruct").field("cell", &self.cell).finish()
        }
    }

    let test_instance = TestStruct { cell: 0 };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", test_instance);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "TestStruct { cell: 0 }");
}

#[test]
fn test_fmt_with_negative_cell() {
    struct TestStruct {
        cell: i32,
    }

    impl core::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TestStruct").field("cell", &self.cell).finish()
        }
    }

    let test_instance = TestStruct { cell: -1 };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", test_instance);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "TestStruct { cell: -1 }");
}

#[test]
fn test_fmt_with_maximum_cell_value() {
    struct TestStruct {
        cell: i32,
    }

    impl core::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TestStruct").field("cell", &self.cell).finish()
        }
    }

    let test_instance = TestStruct { cell: i32::MAX };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", test_instance);
    
    assert!(result.is_ok());
    assert_eq!(buffer, format!("TestStruct {{ cell: {} }}", i32::MAX));
}

#[test]
fn test_fmt_with_minimum_cell_value() {
    struct TestStruct {
        cell: i32,
    }

    impl core::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TestStruct").field("cell", &self.cell).finish()
        }
    }

    let test_instance = TestStruct { cell: i32::MIN };
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", test_instance);
    
    assert!(result.is_ok());
    assert_eq!(buffer, format!("TestStruct {{ cell: {} }}", i32::MIN));
}

