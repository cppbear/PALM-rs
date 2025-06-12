// Answer 0

#[test]
fn test_get_mut_initially_uninitialized() {
    struct Dummy;
    let mut lazy: Lazy<i32, fn() -> i32> = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };
    
    assert_eq!(Lazy::get_mut(&mut lazy), None);
}

#[test]
fn test_get_mut_after_initialization() {
    struct Dummy;
    let mut lazy: Lazy<i32, fn() -> i32> = Lazy {
        cell: OnceCell::with_value(42),
        init: Cell::new(None),
    };
    
    let value_mut = Lazy::get_mut(&mut lazy).unwrap();
    *value_mut = 100;
    
    assert_eq!(*value_mut, 100);
    assert_eq!(Lazy::get_mut(&mut lazy).unwrap(), &mut 100);
}

