// Answer 0

#[test]
fn test_set_returns_err_when_full() {
    static CELL: OnceCell<i32> = OnceCell::new();
    
    // First, set the cell to a value
    CELL.set(42).unwrap();
    
    // Now attempt to set the cell again, which should return Err with the value we tried to set
    let result = CELL.set(100);
    // Here, we would expect Err(100) because the cell is already initialized with 42
}

#[test]
fn test_set_err_with_builtin_type() {
    static CELL: OnceCell<String> = OnceCell::new();
    
    // First, set the cell to a value
    CELL.set("hello".to_string()).unwrap();
    
    // Now attempt to set the cell again, which should return Err with the value we tried to set
    let result = CELL.set("world".to_string());
}

#[test]
fn test_set_err_with_custom_type() {
    struct MyStruct {
        value: i32,
    }

    static CELL: OnceCell<MyStruct> = OnceCell::new();
    
    // First, set the cell to a value
    CELL.set(MyStruct { value: 1 }).unwrap();
    
    // Now attempt to set the cell again, which should return Err with the value we tried to set
    let result = CELL.set(MyStruct { value: 2 });
}

#[test]
fn test_set_err_with_pointer_type() {
    static CELL: OnceCell<*const i32> = OnceCell::new();
    
    let value: i32 = 10;
    let ptr = &value as *const i32;
    
    // First, set the cell to a value
    CELL.set(ptr).unwrap();
    
    // Now attempt to set the cell again, which should return Err with the value we tried to set
    let other_value: i32 = 20;
    let other_ptr = &other_value as *const i32;
    let result = CELL.set(other_ptr);
}

