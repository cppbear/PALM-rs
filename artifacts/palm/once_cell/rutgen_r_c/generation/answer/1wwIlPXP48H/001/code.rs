// Answer 0

#[test]
fn test_lazy_get_uninitialized() {
    struct TestLazy {
        lazy: Lazy<i32>,
    }

    let test_lazy = TestLazy {
        lazy: Lazy::new(|| 42),
    };

    assert_eq!(Lazy::get(&test_lazy.lazy), None);
}

#[test]
fn test_lazy_get_initialized() {
    struct TestLazy {
        lazy: Lazy<i32>,
    }

    let mut test_lazy = TestLazy {
        lazy: Lazy::new(|| 42),
    };

    // Force the initialization
    let _value = Lazy::get_or_init(&mut test_lazy.lazy, || 42);
    
    assert_eq!(Lazy::get(&test_lazy.lazy), Some(&42));
}

#[test]
fn test_lazy_get_multiple_calls() {
    struct TestLazy {
        lazy: Lazy<i32>,
    }

    let mut test_lazy = TestLazy {
        lazy: Lazy::new(|| 42),
    };

    // Initialize once
    let _value = Lazy::get_or_init(&mut test_lazy.lazy, || 42);
    
    // Call get multiple times
    assert_eq!(Lazy::get(&test_lazy.lazy), Some(&42));
    assert_eq!(Lazy::get(&test_lazy.lazy), Some(&42));
}

