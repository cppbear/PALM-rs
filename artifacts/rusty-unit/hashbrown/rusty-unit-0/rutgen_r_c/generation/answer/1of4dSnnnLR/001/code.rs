// Answer 0

#[test]
fn test_into_inner_with_simple_guard() {
    struct TestGuard {
        value: i32,
    }

    let value = 42;
    let guard = ScopeGuard {
        dropfn: |_: &mut i32| {},
        value,
    };
  
    let result = ScopeGuard::into_inner(guard);
    assert_eq!(result, 42);
}

#[test]
fn test_into_inner_with_closure() {
    struct TestGuard {
        value: String,
    }

    let value = String::from("Hello");
    let mut drop_called = false;

    let guard = ScopeGuard {
        dropfn: |s: &mut String| {
            drop_called = true; // to check if dropfn was called
            s.clear();
        },
        value,
    };
  
    let result = ScopeGuard::into_inner(guard);
    assert_eq!(result, "Hello");
    assert!(drop_called);
}

#[test]
fn test_into_inner_with_zero_sized_type() {
    struct TestGuard {
        value: (),
    }

    let value = ();
    let guard = ScopeGuard {
        dropfn: |_: &mut ()| {},
        value,
    };
  
    let result = ScopeGuard::into_inner(guard);
    assert_eq!(result, ());
}

#[should_panic]
fn test_into_inner_with_no_dropfn() {
    struct TestGuard {
        value: usize,
    }

    let guard = ScopeGuard {
        dropfn: |_: &mut usize| panic!("Panic from dropfn"),
        value: 100,
    };

    let _result = ScopeGuard::into_inner(guard);
}

