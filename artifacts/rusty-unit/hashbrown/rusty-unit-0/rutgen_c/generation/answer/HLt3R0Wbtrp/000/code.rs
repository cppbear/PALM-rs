// Answer 0

#[test]
fn test_deref_mut() {
    struct TestGuard {
        data: i32,
    }

    let mut guard = ScopeGuard {
        dropfn: |value: &mut TestGuard| {
            value.data += 1;
        },
        value: TestGuard { data: 0 },
    };

    {
        let value: &mut TestGuard = guard.deref_mut();
        value.data += 10;
    }

    (guard.dropfn)(&mut guard.value);
    assert_eq!(guard.value.data, 11);
}

#[test]
fn test_deref_mut_with_zero_initialization() {
    struct ZeroGuard {
        data: i32,
    }

    let mut guard = ScopeGuard {
        dropfn: |value: &mut ZeroGuard| {
            value.data += 5;
        },
        value: ZeroGuard { data: 0 },
    };

    {
        let value: &mut ZeroGuard = guard.deref_mut();
        value.data = 0;
    }

    (guard.dropfn)(&mut guard.value);
    assert_eq!(guard.value.data, 5);
}

