// Answer 0

#[test]
fn test_force_mut_initialization() {
    struct TestLazy {
        value: Lazy<i32, fn() -> i32>,
    }

    let mut test_lazy = TestLazy {
        value: Lazy {
            cell: OnceCell::new(),
            init: Cell::new(Some(|| 42)),
        },
    };

    let value_ref = Lazy::force_mut(&mut test_lazy.value);
    assert_eq!(*value_ref, 42);
}

#[test]
fn test_force_mut_double_initialization() {
    struct TestLazy {
        value: Lazy<i32, fn() -> i32>,
    }

    let mut test_lazy = TestLazy {
        value: Lazy {
            cell: OnceCell::new(),
            init: Cell::new(Some(|| 99)),
        },
    };

    let first_value_ref = Lazy::force_mut(&mut test_lazy.value);
    assert_eq!(*first_value_ref, 99);

    let second_value_ref = Lazy::force_mut(&mut test_lazy.value);
    assert_eq!(first_value_ref, second_value_ref);
}

#[should_panic(expected = "Lazy instance has previously been poisoned")]
#[test]
fn test_force_mut_poisoning() {
    struct TestLazy {
        value: Lazy<i32, fn() -> i32>,
    }

    let mut test_lazy = TestLazy {
        value: Lazy {
            cell: OnceCell::new(),
            init: Cell::new(None),
        },
    };

    Lazy::force_mut(&mut test_lazy.value);
}

