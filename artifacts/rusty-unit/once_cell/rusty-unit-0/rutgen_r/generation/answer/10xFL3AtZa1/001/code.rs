// Answer 0

#[test]
fn test_into_value_initialized() {
    struct DummyInit;

    struct DummyCell {
        value: Option<i32>,
    }

    impl DummyCell {
        fn new(value: i32) -> Self {
            Self { value: Some(value) }
        }

        fn into_inner(self) -> Option<i32> {
            self.value
        }
    }

    struct Lazy<T, F> {
        cell: DummyCell,
        init: Option<F>,
    }

    let initialized_lazy = Lazy {
        cell: DummyCell::new(42),
        init: None,
    };

    let result: Result<i32, DummyInit> = into_value(initialized_lazy);
    assert_eq!(result, Ok(42));
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_into_value_uninitialized() {
    struct DummyInit;

    struct DummyCell {
        value: Option<i32>,
    }

    impl DummyCell {
        fn new(value: Option<i32>) -> Self {
            Self { value }
        }

        fn into_inner(self) -> Option<i32> {
            self.value
        }
    }

    struct Lazy<T, F> {
        cell: DummyCell,
        init: Option<F>,
    }

    let uninitialized_lazy = Lazy {
        cell: DummyCell::new(None),
        init: Some(DummyInit),
    };

    let _result: Result<i32, DummyInit> = into_value(uninitialized_lazy);
}

#[test]
fn test_into_value_with_non_empty_initialization() {
    struct DummyInit;

    struct DummyCell {
        value: Option<i32>,
    }

    impl DummyCell {
        fn new(value: Option<i32>) -> Self {
            Self { value }
        }

        fn into_inner(self) -> Option<i32> {
            self.value
        }
    }

    struct Lazy<T, F> {
        cell: DummyCell,
        init: Option<F>,
    }

    let uninitialized_lazy_with_panicked_init = Lazy {
        cell: DummyCell::new(None),
        init: Some(DummyInit),
    };

    let result: Result<i32, DummyInit> = into_value(uninitialized_lazy_with_panicked_init);
    assert_eq!(result.err().is_some(), true);
}

