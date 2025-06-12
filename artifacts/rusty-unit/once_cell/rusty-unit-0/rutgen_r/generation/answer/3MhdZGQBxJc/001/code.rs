// Answer 0

fn test_fmt_with_initialized_value() {
    use once_cell::sync::OnceCell;
    use std::fmt;

    struct TestStruct {
        value: i32,
    }

    impl fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("TestStruct")
                .field("value", &self.value)
                .finish()
        }
    }

    let cell: OnceCell<TestStruct> = OnceCell::new();
    cell.set(TestStruct { value: 42 }).unwrap();

    let result = format!("{:?}", cell);
    assert_eq!(result, "OnceCell(42)");
}

fn test_fmt_with_uninitialized_value() {
    use once_cell::sync::OnceCell;
    use std::fmt;

    let cell: OnceCell<i32> = OnceCell::new();

    let result = format!("{:?}", cell);
    assert_eq!(result, "OnceCell(Uninit)");
}

fn test_fmt_with_multiple_initializations() {
    use once_cell::sync::OnceCell;
    use std::fmt;

    let cell: OnceCell<i32> = OnceCell::new();
    cell.set(100).unwrap();

    let result = format!("{:?}", cell);
    assert_eq!(result, "OnceCell(100)");
}

fn test_fmt_with_boundary_value() {
    use once_cell::sync::OnceCell;
    use std::fmt;

    let cell: OnceCell<u32> = OnceCell::new();
    cell.set(0).unwrap(); // testing zero as a boundary value

    let result = format!("{:?}", cell);
    assert_eq!(result, "OnceCell(0)");
}

