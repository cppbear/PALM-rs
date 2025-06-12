// Answer 0

#[test]
fn test_force_mut_with_initialized_cell() {
    struct TestStruct {
        value: i32,
    }
    
    let mut lazy = Lazy {
        cell: OnceCell::with_value(TestStruct { value: 42 }),
        init: Cell::new(Some(|| TestStruct { value: 0 })),
    };
    
    let result: &mut TestStruct = Lazy::force_mut(&mut lazy);
}

#[test]
#[should_panic]
fn test_force_mut_with_poisoned_lazy() {
    struct TestStruct {
        value: i32,
    }
    
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };
    
    let _result: &mut TestStruct = Lazy::force_mut(&mut lazy);
}

#[test]
fn test_force_mut_with_multiple_calls() {
    struct TestStruct {
        value: i32,
    }
    
    let mut lazy = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| TestStruct { value: 99 })),
    };
    
    let first_result: &mut TestStruct = Lazy::force_mut(&mut lazy);
    let second_result: &mut TestStruct = Lazy::force_mut(&mut lazy);

    assert_eq!(first_result.value, second_result.value);
}

