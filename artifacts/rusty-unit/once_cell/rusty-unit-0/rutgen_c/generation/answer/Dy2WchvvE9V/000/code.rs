// Answer 0

#[test]
fn test_lazy_force_initialization() {
    struct TestLazy {
        cell: OnceCell<i32>,
        init: Cell<Option<fn() -> i32>>,
    }
    
    impl TestLazy {
        fn new(init: fn() -> i32) -> Self {
            TestLazy {
                cell: OnceCell(Imp { queue: AtomicPtr::new(std::ptr::null_mut()), value: UnsafeCell::new(None) }),
                init: Cell::new(Some(init)),
            }
        }
        
        fn force(&self) -> &i32 {
            Lazy::force(self)
        }
    }
    
    let lazy = TestLazy::new(|| 42);
    assert_eq!(*lazy.force(), 42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_lazy_force_poisoning() {
    struct TestLazy {
        cell: OnceCell<i32>,
        init: Cell<Option<fn() -> i32>>,
    }
    
    impl TestLazy {
        fn new(init: fn() -> i32) -> Self {
            TestLazy {
                cell: OnceCell(Imp { queue: AtomicPtr::new(std::ptr::null_mut()), value: UnsafeCell::new(None) }),
                init: Cell::new(Some(init)),
            }
        }
        
        fn force(&mut self) -> &i32 {
            Lazy::force(self)
        }
    }
    
    let mut lazy = TestLazy::new(|| 42);
    assert_eq!(*lazy.force(), 42);
    lazy.init.set(None); // Simulating poisoning
    let _ = lazy.force(); // This should panic
}

#[test]
fn test_lazy_force_multiple_calls() {
    struct TestLazy {
        cell: OnceCell<i32>,
        init: Cell<Option<fn() -> i32>>,
    }
    
    impl TestLazy {
        fn new(init: fn() -> i32) -> Self {
            TestLazy {
                cell: OnceCell(Imp { queue: AtomicPtr::new(std::ptr::null_mut()), value: UnsafeCell::new(None) }),
                init: Cell::new(Some(init)),
            }
        }
        
        fn force(&self) -> &i32 {
            Lazy::force(self)
        }
    }
    
    let lazy = TestLazy::new(|| 99);
    assert_eq!(*lazy.force(), 99);
    assert_eq!(*lazy.force(), 99); // Calling again should yield the same result
}

