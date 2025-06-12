// Answer 0

#[test]
fn test_get_uninitialized() {
    struct TestLazy {
        cell: OnceCell<i32>,
        init: Cell<Option<fn() -> i32>>,
    }
    
    let lazy = TestLazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| 92)),
    };
    
    assert_eq!(Lazy::get(&lazy), None);
}

#[test]
fn test_get_initialized() {
    struct TestLazy {
        cell: OnceCell<i32>,
        init: Cell<Option<fn() -> i32>>,
    }

    let mut lazy = TestLazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| 92)),
    };

    lazy.cell.set(92).unwrap();
    
    assert_eq!(Lazy::get(&lazy), Some(&92));
}

#[test]
fn test_get_after_force_initialization() {
    struct TestLazy {
        cell: OnceCell<i32>,
        init: Cell<Option<fn() -> i32>>,
    }

    let lazy = TestLazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| 92)),
    };

    // Simulate force initialization by setting the value directly
    let _ = lazy.cell.set(92);

    assert_eq!(Lazy::get(&lazy), Some(&92));
}

#[test]
fn test_get_multiple_times() {
    struct TestLazy {
        cell: OnceCell<i32>,
        init: Cell<Option<fn() -> i32>>,
    }

    let mut lazy = TestLazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| 92)),
    };

    lazy.cell.set(92).unwrap();

    assert_eq!(Lazy::get(&lazy), Some(&92));
    assert_eq!(Lazy::get(&lazy), Some(&92));
}

