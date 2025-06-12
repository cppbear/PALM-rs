// Answer 0

#[derive(Debug)]
struct TestFn;

struct Lazy<T, F> {
    cell: CellOption<T>,
    init: Option<F>,
}

struct CellOption<T> {
    value: Option<T>,
}

impl<T> CellOption<T> {
    fn into_inner(self) -> Option<T> {
        self.value
    }
}

#[test]
fn test_into_value_success() {
    let value = 42;
    let init_fn = TestFn;
    let lazy = Lazy {
        cell: CellOption { value: Some(value) },
        init: Some(init_fn),
    };
    
    let result: Result<i32, TestFn> = into_value(lazy);
    assert_eq!(result, Ok(42));
}

#[test]
#[should_panic(expected = "Lazy instance has previously been poisoned")]
fn test_into_value_fail_poisoned() {
    let init_fn = TestFn;
    let lazy = Lazy {
        cell: CellOption { value: None },
        init: Some(init_fn),
    };
    
    let _result: Result<i32, TestFn> = into_value(lazy);
}

