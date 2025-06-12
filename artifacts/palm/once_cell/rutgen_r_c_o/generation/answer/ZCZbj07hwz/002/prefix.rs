// Answer 0

#[test]
fn test_force_mut_with_none_cell_and_none_init() {
    struct TestStruct;
    let mut lazy: Lazy<TestStruct> = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };
    let result = Lazy::force_mut(&mut lazy);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_with_none_cell_and_some_init_panic() {
    struct TestStruct;
    let mut lazy: Lazy<TestStruct> = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| TestStruct)),
    };
    let _ = Lazy::force_mut(&mut lazy);
}

#[test]
fn test_force_mut_twice_with_none_cell_and_some_init() {
    struct TestStruct;
    let mut lazy: Lazy<TestStruct> = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| TestStruct)),
    };
    
    let result1 = Lazy::force_mut(&mut lazy);
    let result2 = Lazy::force_mut(&mut lazy);
}

