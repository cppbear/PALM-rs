// Answer 0

#[test]
fn test_get_or_try_init_success() {
    struct TestValue(i32);
    let cell = OnceCell::new();
    let result = cell.get_or_try_init(|| Ok(TestValue(92)));
    assert_eq!(result, Ok(&TestValue(92)));
    assert_eq!(cell.get(), Some(&TestValue(92)));
}

#[test]
fn test_get_or_try_init_failure() {
    struct TestValue(i32);
    let cell = OnceCell::new();
    let result: Result<&TestValue, ()> = cell.get_or_try_init(|| Err(()));
    assert_eq!(result, Err(()));
    assert!(cell.get().is_none());
}

#[test]
#[should_panic]
fn test_get_or_try_init_panic() {
    struct TestValue(i32);
    let cell = OnceCell::new();
    let _ = cell.get_or_try_init(|| {
        panic!("Expected panic");
    });
}

#[test]
fn test_get_or_try_init_reentrant_initialization() {
    struct TestValue(i32);
    let cell = OnceCell::new();
    let _ = cell.get_or_try_init(|| {
        let _ = cell.get_or_try_init(|| Ok(TestValue(92)));
        Ok(TestValue(84))
    });
    assert!(cell.get().is_none());
}

