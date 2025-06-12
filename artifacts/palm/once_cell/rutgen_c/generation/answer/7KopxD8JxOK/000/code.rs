// Answer 0

#[test]
fn test_once_cell_debug_initialized() {
    struct TestStruct {
        value: i32,
    }
    
    let cell = OnceCell::with_value(TestStruct { value: 42 });
    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        cell.fmt(&mut formatter).unwrap();
    }
    assert_eq!(output, "OnceCell(42)");
}

#[test]
fn test_once_cell_debug_uninitialized() {
    let cell: OnceCell<i32> = OnceCell::new();
    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        cell.fmt(&mut formatter).unwrap();
    }
    assert_eq!(output, "OnceCell(Uninit)");
}

