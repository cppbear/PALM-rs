// Answer 0

#[test]
fn test_lazy_get_before_initialization() {
    struct TestLazy;
    
    let lazy: Lazy<i32, fn() -> i32> = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };
    
    assert_eq!(Lazy::get(&lazy), None);
}

#[test]
fn test_lazy_get_after_initialization() {
    struct TestLazy;
    
    let lazy: Lazy<i32, fn() -> i32> = Lazy {
        cell: OnceCell::with_value(42),
        init: Cell::new(None),
    };
    
    assert_eq!(Lazy::get(&lazy), Some(&42));
}

#[test]
fn test_lazy_get_multi_initialization() {
    struct TestLazy;
    
    let lazy: Lazy<i32, fn() -> i32> = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };
    
    // Simulate initialization
    lazy.cell.set(100).unwrap();
    
    assert_eq!(Lazy::get(&lazy), Some(&100));
}

