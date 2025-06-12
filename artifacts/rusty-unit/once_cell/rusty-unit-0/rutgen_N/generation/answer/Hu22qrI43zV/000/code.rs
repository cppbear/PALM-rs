// Answer 0

#[derive(Default)]
struct DummyInit;
struct Lazy<T, F> {
    cell: Option<T>,
    init: Option<F>,
}

impl<T, F> Lazy<T, F> {
    fn new() -> Self {
        Self {
            cell: None,
            init: Some(DummyInit::default()),
        }
    }

    fn init(&mut self, value: T) {
        self.cell = Some(value);
        self.init = None;
    }

    fn into_inner(self) -> Result<T, F> {
        self.cell.ok_or_else(|| self.init.take().unwrap())
    }
}

#[test]
fn test_into_value_initialized() {
    let mut lazy = Lazy::new();
    lazy.init(42);

    let result: Result<i32, DummyInit> = into_value(lazy);

    assert_eq!(result, Ok(42));
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_into_value_uninitialized() {
    let lazy = Lazy::<i32, DummyInit>::new();

    let result: Result<i32, DummyInit> = into_value(lazy);

    assert!(result.is_err());
}

