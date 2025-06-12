// Answer 0

#[test]
fn test_once_cell_debug_initialized() {
    struct TestStruct {
        value: i32,
    }
    
    let cell = OnceCell::with_value(TestStruct { value: 42 });
    let debug_string = format!("{:?}", cell);
    
    assert_eq!(debug_string, "OnceCell(OnceCell { value: 42 })");
}

#[test]
fn test_once_cell_debug_uninitialized() {
    struct TestStruct {
        value: i32,
    }
    
    let cell: OnceCell<TestStruct> = OnceCell::new();
    let debug_string = format!("{:?}", cell);
    
    assert_eq!(debug_string, "OnceCell(Uninit)");
}

