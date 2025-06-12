// Answer 0

#[test]
fn test_lazy_force_value_initialization() {
    struct Lazy<T, F> {
        cell: Cell<T>,
        init: Option<F>,
    }
    
    struct Cell<T> {
        value: Option<T>,
    }
    
    impl<T> Cell<T> {
        fn new() -> Self {
            Cell { value: None }
        }
        
        fn get_or_init<F>(&mut self, init: F) -> &T
        where
            F: FnOnce() -> T,
        {
            if self.value.is_none() {
                self.value = Some(init());
            }
            self.value.as_ref().unwrap()
        }
    }
    
    let lazy = Lazy {
        cell: Cell::new(),
        init: Some(|| 42),
    };

    assert_eq!(*force(&lazy), 42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_lazy_force_panic_on_poisoning() {
    struct Lazy<T, F> {
        cell: Cell<T>,
        init: Option<F>,
    }
    
    struct Cell<T> {
        value: Option<T>,
    }
    
    impl<T> Cell<T> {
        fn new() -> Self {
            Cell { value: None }
        }
        
        fn get_or_init<F>(&mut self, init: F) -> &T
        where
            F: FnOnce() -> T,
        {
            if self.value.is_none() {
                self.value = Some(init());
            }
            self.value.as_ref().unwrap()
        }
    }
    
    let lazy = Lazy {
        cell: Cell::new(),
        init: None,
    };

    force(&lazy);
}

