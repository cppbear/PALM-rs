// Answer 0

#[test]
fn test_force_mut_empty_cell_and_init() {
    struct TestStruct;
    
    let mut lazy: Lazy<TestStruct, fn() -> TestStruct> = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };
    
    let result = Lazy::force_mut(&mut lazy);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_mut_poisoned_instance() {
    struct TestStruct;
    
    let mut lazy: Lazy<TestStruct, fn() -> TestStruct> = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };
    
    // Simulate a poisoned state by prematurely setting the init to None
    lazy.init.set(None);
    
    let _result = Lazy::force_mut(&mut lazy);
}

