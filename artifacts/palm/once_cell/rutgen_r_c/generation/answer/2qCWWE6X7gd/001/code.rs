// Answer 0

#[test]
fn test_get_when_cell_is_empty() {
    let cell: OnceCell<i32> = OnceCell::new();
    let result = cell.get();
    assert!(result.is_none());
}

#[test]
fn test_get_after_set() {
    let cell = OnceCell::new();
    let _ = cell.set(42).unwrap();
    let result = cell.get();
    assert_eq!(result, Some(&42));
}

#[test]
fn test_get_or_init_with_initialization() {
    let cell = OnceCell::new();
    let result = cell.get_or_init(|| 100);
    assert_eq!(result, &100);
    let result_after = cell.get();
    assert_eq!(result_after, Some(&100));
}

#[test]
fn test_get_mut_after_set() {
    let cell = OnceCell::new();
    let _ = cell.set(42).unwrap();
    let result = cell.get_mut();
    assert_eq!(result, Some(&mut 42));
}

#[test]
fn test_get_or_try_init_success() {
    let cell = OnceCell::new();
    let result = cell.get_or_try_init(|| Ok(50));
    assert_eq!(result, Ok(&50));
    assert_eq!(cell.get(), Some(&50));
}

