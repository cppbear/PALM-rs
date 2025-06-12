// Answer 0

#[test]
fn test_force_mut_initializes_value() {
    let mut lazy: Lazy<u32, fn() -> u32> = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(Some(|| 42)),
    };
    
    let value: &mut u32 = Lazy::force_mut(&mut lazy);
    assert_eq!(*value, 42);
}

#[test]
fn test_force_mut_returns_mut_value() {
    let mut lazy: Lazy<u32, fn() -> u32> = Lazy {
        cell: OnceCell::with_value(100),
        init: Cell::new(None),
    };
    
    let value: &mut u32 = Lazy::force_mut(&mut lazy);
    assert_eq!(*value, 100);
    
    *value += 1;
    assert_eq!(*value, 101);
}

#[should_panic(expected = "Lazy instance has previously been poisoned")]
#[test]
fn test_force_mut_panics_on_poisoned_init() {
    let mut lazy: Lazy<u32, fn() -> u32> = Lazy {
        cell: OnceCell::new(),
        init: Cell::new(None),
    };
    
    Lazy::force_mut(&mut lazy);
}

#[test]
fn test_force_mut_retains_value() {
    let mut lazy: Lazy<u32, fn() -> u32> = Lazy {
        cell: OnceCell::with_value(10),
        init: Cell::new(None),
    };
    
    let value: &mut u32 = Lazy::force_mut(&mut lazy);
    assert_eq!(*value, 10);
    
    let second_value: &mut u32 = Lazy::force_mut(&mut lazy);
    assert_eq!(value, second_value);
}

