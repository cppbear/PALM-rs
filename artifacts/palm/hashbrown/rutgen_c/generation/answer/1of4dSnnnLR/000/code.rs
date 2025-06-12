// Answer 0

#[test]
fn test_into_inner_with_integer() {
    let guard = ScopeGuard {
        dropfn: |x: &mut i32| *x += 1,
        value: 42,
    };
    let value = ScopeGuard::into_inner(guard);
    assert_eq!(value, 42);
}

#[test]
fn test_into_inner_with_string() {
    let guard = ScopeGuard {
        dropfn: |s: &mut String| s.push_str(" world"),
        value: String::from("Hello"),
    };
    let value = ScopeGuard::into_inner(guard);
    assert_eq!(value, "Hello");
}

#[test]
fn test_into_inner_with_empty_string() {
    let guard = ScopeGuard {
        dropfn: |s: &mut String| s.push_str("!"),
        value: String::from(""),
    };
    let value = ScopeGuard::into_inner(guard);
    assert_eq!(value, "");
}

