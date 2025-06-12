// Answer 0

#[test]
fn test_try_insert_initial_empty() {
    struct TestOnceCell(Imp<i32>);
    let cell = OnceCell::<i32>::new();
    assert!(cell.get().is_none());

    assert_eq!(cell.try_insert(92), Ok(&92));
    assert_eq!(cell.get(), Some(&92));
}

#[test]
fn test_try_insert_with_value() {
    struct TestOnceCell(Imp<i32>);
    let cell = OnceCell::<i32>::new();
    assert_eq!(cell.try_insert(42), Ok(&42));
    
    assert_eq!(cell.try_insert(62), Err((&42, 62)));
    assert_eq!(cell.get(), Some(&42));
}

#[test]
fn test_try_insert_multiple_calls() {
    struct TestOnceCell(Imp<i32>);
    let cell = OnceCell::<i32>::new();
    assert!(cell.get().is_none());

    assert_eq!(cell.try_insert(100), Ok(&100));
    assert_eq!(cell.try_insert(200), Err((&100, 200)));
    assert_eq!(cell.try_insert(300), Err((&100, 300)));
    assert_eq!(cell.get(), Some(&100));
}

#[test]
fn test_try_insert_with_different_values() {
    struct TestOnceCell(Imp<String>);
    let cell = OnceCell::<String>::new();
    assert!(cell.get().is_none());

    assert_eq!(cell.try_insert("Hello".to_string()), Ok(&"Hello".to_string()));
    assert_eq!(cell.try_insert("World".to_string()), Err((&"Hello".to_string(), "World".to_string())));
    assert_eq!(cell.get(), Some(&"Hello".to_string()));
}

