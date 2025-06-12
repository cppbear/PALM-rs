// Answer 0

#[test]
fn test_get_or_try_init_success() {
    struct CellWrapper {
        once_cell: OnceCell<i32>,
    }
    
    let wrapper = CellWrapper {
        once_cell: OnceCell::new(),
    };

    let result = wrapper.once_cell.get_or_try_init(|| Ok(42));
    assert_eq!(result, Ok(&42));
    assert_eq!(wrapper.once_cell.get(), Some(&42));
}

#[test]
fn test_get_or_try_init_failure() {
    struct CellWrapper {
        once_cell: OnceCell<i32>,
    }
    
    let wrapper = CellWrapper {
        once_cell: OnceCell::new(),
    };

    let result: Result<_, ()> = wrapper.once_cell.get_or_try_init(|| Err(()));
    assert_eq!(result, Err(()));
    assert!(wrapper.once_cell.get().is_none());
}

#[test]
#[should_panic]
fn test_get_or_try_init_reentrant_init() {
    struct CellWrapper {
        once_cell: OnceCell<i32>,
    }

    let mut wrapper = CellWrapper {
        once_cell: OnceCell::new(),
    };

    let _ = wrapper.once_cell.get_or_try_init(|| {
        wrapper.once_cell.get_or_try_init(|| Ok(10)).unwrap();
        Ok(20)
    });
}

#[test]
fn test_get_or_try_init_initialization_already_done() {
    struct CellWrapper {
        once_cell: OnceCell<i32>,
    }

    let mut wrapper = CellWrapper {
        once_cell: OnceCell::new(),
    };

    let _ = wrapper.once_cell.get_or_try_init(|| Ok(30));
    let result = wrapper.once_cell.get_or_try_init(|| Err(()));
    
    assert_eq!(result, Ok(&30));
    assert!(wrapper.once_cell.get().is_some());
}

