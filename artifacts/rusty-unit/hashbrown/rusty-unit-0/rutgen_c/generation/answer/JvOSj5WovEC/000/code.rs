// Answer 0

#[test]
fn test_deref_scope_guard() {
    struct ExampleGuard {
        data: i32,
    }

    let mut guard = ScopeGuard {
        dropfn: |value: &mut ExampleGuard| {
            value.data += 1;
        },
        value: ExampleGuard { data: 5 },
    };

    assert_eq!(*guard, ExampleGuard { data: 5 });
}

#[test]
fn test_deref_multiple_access() {
    struct Counter {
        count: i32,
    }

    let mut guard = ScopeGuard {
        dropfn: |value: &mut Counter| {
            value.count += 1;
        },
        value: Counter { count: 10 },
    };

    assert_eq!(*guard, Counter { count: 10 });
    assert_eq!(*guard, Counter { count: 10 });
}

