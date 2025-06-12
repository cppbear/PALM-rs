// Answer 0

#[test]
fn test_set_once_bool_empty() {
    let once_bool = OnceBool::new();
    assert_eq!(once_bool.set(true), Ok(()));
    assert_eq!(once_bool.get(), Some(true));
}

#[test]
fn test_set_once_bool_full() {
    let once_bool = OnceBool::new();
    once_bool.set(false).unwrap();
    assert_eq!(once_bool.set(true), Err(()));
    assert_eq!(once_bool.get(), Some(false));
}

#[test]
fn test_set_once_bool_multiple_calls() {
    let once_bool = OnceBool::new();
    once_bool.set(true).unwrap();
    assert_eq!(once_bool.set(false), Err(()));
    assert_eq!(once_bool.set(false), Err(()));
    assert_eq!(once_bool.get(), Some(true));
}

#[test]
fn test_set_ordering() {
    let once_bool = OnceBool::new();
    assert_eq!(once_bool.set(true), Ok(()));
    assert_eq!(once_bool.get(), Some(true));
    once_bool.set(false).unwrap_err();
    assert_eq!(once_bool.get(), Some(true));
}

