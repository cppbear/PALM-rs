// Answer 0

#[inline(always)]
struct OnceCell<T> {
    inner: std::cell::UnsafeCell<Option<T>>,
}

#[test]
fn test_once_cell_creation() {
    let cell: OnceCell<i32> = new();
    assert!(unsafe { (*cell.inner.get()).is_none() });
}

#[test]
fn test_once_cell_creation_with_different_types() {
    let cell: OnceCell<String> = new();
    assert!(unsafe { (*cell.inner.get()).is_none() });
}

