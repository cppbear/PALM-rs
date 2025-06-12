// Answer 0

#[test]
fn test_force_with_initialization() {
    struct DummyCell {
        value: Option<i32>,
    }

    impl DummyCell {
        fn new() -> Self {
            DummyCell { value: None }
        }

        fn get_or_init<F>(&mut self, init: F) -> &i32
        where
            F: FnOnce() -> i32,
        {
            if self.value.is_none() {
                self.value = Some(init());
            }
            self.value.as_ref().unwrap()
        }
    }

    struct Lazy<F> {
        cell: DummyCell,
        init: Option<F>,
    }

    impl<F> Lazy<F>
    where
        F: FnOnce() -> i32,
    {
        fn new(init: F) -> Self {
            Lazy {
                cell: DummyCell::new(),
                init: Some(init),
            }
        }
    }

    let mut lazy = Lazy::new(|| 42);
    assert_eq!(*force(&lazy), 42);
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_force_with_poisoned_lazy() {
    struct DummyCell {
        value: Option<i32>,
    }

    impl DummyCell {
        fn new() -> Self {
            DummyCell { value: None }
        }

        fn get_or_init<F>(&mut self, init: F) -> &i32
        where
            F: FnOnce() -> i32,
        {
            if self.value.is_none() {
                self.value = Some(init());
            }
            self.value.as_ref().unwrap()
        }
    }

    struct Lazy<F> {
        cell: DummyCell,
        init: Option<F>,
    }

    impl<F> Lazy<F>
    where
        F: FnOnce() -> i32,
    {
        fn new(init: F) -> Self {
            Lazy {
                cell: DummyCell::new(),
                init: Some(init),
            }
        }
    }

    let mut lazy = Lazy::new(|| 42);
    let _ = force(&lazy); // Initialize it once to set the value
    lazy.init = None; // Simulate poisoning
    let _ = force(&lazy); // This should panic
}

