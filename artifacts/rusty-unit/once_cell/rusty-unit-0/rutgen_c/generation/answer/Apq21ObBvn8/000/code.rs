// Answer 0

#[test]
fn test_get_mut_initially_none() {
    struct Dummy;
    let mut lazy = Lazy::<i32, fn() -> i32>::new(|| 92);
    assert_eq!(Lazy::get_mut(&mut lazy), None);
}

#[test]
fn test_get_mut_after_initialization() {
    struct Dummy;
    let mut lazy = Lazy::<i32, fn() -> i32>::new(|| 92);
    let value = Lazy::force_mut(&mut lazy);
    assert_eq!(Lazy::get_mut(&mut lazy), Some(value));
}

#[test]
fn test_get_mut_after_force_mut() {
    struct Dummy;
    let mut lazy = Lazy::<i32, fn() -> i32>::new(|| 92);
    let value = Lazy::force_mut(&mut lazy);
    *value += 8;
    assert_eq!(Lazy::get_mut(&mut lazy), Some(value));
    assert_eq!(*value, 100);
}

