// Answer 0

#[test]
fn test_set_empty_cell() {
    struct TestOnceCell {
        cell: OnceCell<i32>,
    }

    let test_cell = TestOnceCell {
        cell: OnceCell::new(),
    };
    
    assert_eq!(test_cell.cell.set(42), Ok(()));
}

#[test]
fn test_set_non_empty_cell() {
    struct TestOnceCell {
        cell: OnceCell<i32>,
    }

    let mut test_cell = TestOnceCell {
        cell: OnceCell::new(),
    };

    assert_eq!(test_cell.cell.set(42), Ok(()));
    assert_eq!(test_cell.cell.set(58), Err(58));
}

#[test]
fn test_set_and_get() {
    struct TestOnceCell {
        cell: OnceCell<i32>,
    }

    let mut test_cell = TestOnceCell {
        cell: OnceCell::new(),
    };

    assert_eq!(test_cell.cell.set(100), Ok(()));
    assert_eq!(test_cell.cell.get(), Some(&100));
    assert_eq!(test_cell.cell.set(200), Err(200));
}

#[test]
fn test_multiple_threads() {
    use std::thread;

    struct TestOnceCell {
        cell: OnceCell<i32>,
    }

    let test_cell = TestOnceCell {
        cell: OnceCell::new(),
    };

    let handle = thread::spawn(move || {
        assert_eq!(test_cell.cell.set(10), Ok(()));
    });

    handle.join().unwrap();
    assert_eq!(test_cell.cell.set(20), Err(20));
    assert_eq!(test_cell.cell.get(), Some(&10));
}

