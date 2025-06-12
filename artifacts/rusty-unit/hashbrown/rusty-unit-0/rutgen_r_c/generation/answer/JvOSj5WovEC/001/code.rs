// Answer 0

#[test]
fn test_deref() {
    struct TestDropFn;
    
    let drop_fn = |value: &mut i32| { *value += 1; };
    
    let scope_guard = ScopeGuard {
        dropfn: drop_fn,
        value: 42,
    };
    
    assert_eq!(*scope_guard.deref(), 42);
}

#[test]
fn test_deref_negative_value() {
    struct TestDropFn;
    
    let drop_fn = |value: &mut i32| { *value += 1; };
    
    let scope_guard = ScopeGuard {
        dropfn: drop_fn,
        value: -10,
    };
    
    assert_eq!(*scope_guard.deref(), -10);
}

#[test]
fn test_deref_zero() {
    struct TestDropFn;
    
    let drop_fn = |value: &mut i32| { *value += 1; };
    
    let scope_guard = ScopeGuard {
        dropfn: drop_fn,
        value: 0,
    };
    
    assert_eq!(*scope_guard.deref(), 0);
}

