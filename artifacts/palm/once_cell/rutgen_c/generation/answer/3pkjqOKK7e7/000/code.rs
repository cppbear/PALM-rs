// Answer 0

#[test]
fn test_deref_lazy() {
    struct Dummy;

    let init_fn = || 42;
    let lazy_instance = Lazy {
        cell: OnceCell { 
            inner: UnsafeCell::new(None) 
        },
        init: Cell::new(Some(init_fn)),
    };

    let value: &i32 = lazy_instance.deref();
    assert_eq!(*value, 42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_deref_lazy_poisoned() {
    struct Dummy;

    let init_fn = || 42;
    let mut lazy_instance = Lazy {
        cell: OnceCell { 
            inner: UnsafeCell::new(None) 
        },
        init: Cell::new(Some(init_fn)),
    };

    // Force initialization to set the value
    let _ = lazy_instance.deref();

    // Poison the Lazy by taking the init function
    lazy_instance.init.set(None);

    // This should panic because the lazy instance has been poisoned
    let _value: &i32 = lazy_instance.deref();
}

