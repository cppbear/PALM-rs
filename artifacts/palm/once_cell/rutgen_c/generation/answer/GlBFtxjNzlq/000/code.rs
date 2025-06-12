// Answer 0

#[test]
fn test_get_mut_returns_none_when_empty() {
    struct TestOnceCell<T>(Imp<T>);
    let mut cell: OnceCell<u32> = OnceCell::new();
    let result = cell.get_mut();
    assert!(result.is_none());
}

#[test]
fn test_get_mut_returns_mutable_reference() {
    struct TestOnceCell<T>(Imp<T>);
    let mut cell = OnceCell::with_value(42);
    if let Some(value) = cell.get_mut() {
        *value += 10;
    }
    assert_eq!(cell.get_mut().unwrap(), &mut 52);
}

#[test]
fn test_get_mut_multiple_mut_access() {
    struct TestOnceCell<T>(Imp<T>);
    let mut cell = OnceCell::with_value(100);
    {
        let value1 = cell.get_mut().unwrap();
        *value1 += 1;
    }
    {
        let value2 = cell.get_mut().unwrap();
        *value2 += 2;
    }
    assert_eq!(cell.get_mut().unwrap(), &mut 103);
}

