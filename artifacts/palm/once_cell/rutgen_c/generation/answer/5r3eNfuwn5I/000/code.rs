// Answer 0

#[test]
fn test_set_empty_cell() {
    struct TestOnceCell(Imp<i32>);
    
    let cell = OnceCell::new();
    assert_eq!(cell.set(10), Ok(()));
    assert_eq!(cell.get(), Some(&10));
}

#[test]
fn test_set_full_cell() {
    struct TestOnceCell(Imp<i32>);
    
    let cell = OnceCell::with_value(20);
    assert_eq!(cell.set(30), Err(30));
    assert_eq!(cell.get(), Some(&20));
}

#[test]
fn test_set_cell_with_multiple_threads() {
    use std::thread;

    struct TestOnceCell(Imp<i32>);

    let cell = OnceCell::new();
    let handle = thread::spawn(move || {
        assert_eq!(cell.set(50), Ok(()));
    });
    handle.join().unwrap();

    assert_eq!(cell.set(40), Err(40));
    assert_eq!(cell.get(), Some(&50));
}

