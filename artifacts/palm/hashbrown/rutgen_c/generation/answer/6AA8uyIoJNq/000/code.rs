// Answer 0

#[test]
fn test_guard_creates_scope_guard() {
    let mut value = 42;
    let mut drop_called = false;
    
    let dropfn = |x: &mut i32| {
        *x += 1;
        drop_called = true;
    };
    
    let guard_instance = guard(value, dropfn);
    
    assert_eq!(guard_instance.value, 42);
    assert!(!drop_called);
}

#[test]
fn test_guard_triggers_drop_function() {
    let mut value = 10;
    let mut drop_called = false;

    {
        let dropfn = |x: &mut i32| {
            *x += 5;
            drop_called = true;
        };
        
        let _guard_instance = guard(value, dropfn);
        
        // Drop happens here when the guard_instance goes out of scope
    }
    
    assert_eq!(value, 10);
    assert!(drop_called);
}

#[test]
fn test_guard_with_zero_initial_value() {
    let mut value = 0;
    let mut drop_called = false;

    {
        let dropfn = |x: &mut i32| {
            *x += 10;
            drop_called = true;
        };
        
        let _guard_instance = guard(value, dropfn);
    }
    
    assert_eq!(value, 0);
    assert!(drop_called); 
}

