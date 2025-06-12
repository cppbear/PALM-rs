// Answer 0

#[test]
fn test_lazy_with_u32_uninitialized() {
    let lazy: Lazy<u32> = Lazy {
        cell: OnceCell {
            inner: UnsafeCell::new(None),
        },
        init: Cell::new(None),
    };
    let _ = format!("{:?}", lazy);
}

#[test]
fn test_lazy_with_u32_initialized() {
    let init_fn: fn() -> u32 = || 42;
    let lazy: Lazy<u32, fn() -> u32> = Lazy {
        cell: OnceCell {
            inner: UnsafeCell::new(Some(42)),
        },
        init: Cell::new(Some(init_fn)),
    };
    let _ = format!("{:?}", lazy);
}

#[test]
fn test_lazy_with_string_uninitialized() {
    let lazy: Lazy<String> = Lazy {
        cell: OnceCell {
            inner: UnsafeCell::new(None),
        },
        init: Cell::new(None),
    };
    let _ = format!("{:?}", lazy);
}

#[test]
fn test_lazy_with_string_initialized() {
    let init_fn: fn() -> String = || "Hello".to_string();
    let lazy: Lazy<String, fn() -> String> = Lazy {
        cell: OnceCell {
            inner: UnsafeCell::new(Some("Hello".to_string())),
        },
        init: Cell::new(Some(init_fn)),
    };
    let _ = format!("{:?}", lazy);
}

#[test]
fn test_lazy_with_option_some() {
    let init_fn: fn() -> Option<u32> = || Some(10);
    let lazy: Lazy<Option<u32>, fn() -> Option<u32>> = Lazy {
        cell: OnceCell {
            inner: UnsafeCell::new(Some(Some(10))),
        },
        init: Cell::new(Some(init_fn)),
    };
    let _ = format!("{:?}", lazy);
}

#[test]
fn test_lazy_with_option_none() {
    let init_fn: fn() -> Option<u32> = || None;
    let lazy: Lazy<Option<u32>, fn() -> Option<u32>> = Lazy {
        cell: OnceCell {
            inner: UnsafeCell::new(Some(None)),
        },
        init: Cell::new(Some(init_fn)),
    };
    let _ = format!("{:?}", lazy);
}

