// Answer 0

#[test]
fn test_get_or_init_with_non_empty_cell() {
    use std::num::NonZeroUsize;
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let value = NonZeroUsize::new(5).unwrap();
    
    let result = cell.get_or_init(|| value);
    
    assert_eq!(*result, value);
    assert_eq!(cell.get().unwrap(), &value);
}

#[test]
fn test_get_or_init_with_empty_cell() {
    use std::num::NonZeroUsize;
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    
    let result = cell.get_or_init(|| NonZeroUsize::new(10).unwrap());
    
    assert_eq!(*result, NonZeroUsize::new(10).unwrap());
    assert_eq!(cell.get().unwrap(), &NonZeroUsize::new(10).unwrap());
}

#[test]
fn test_get_or_init_concurrently() {
    use std::num::NonZeroUsize;
    use once_cell::sync::OnceCell;
    use std::sync::Arc;
    use std::thread;

    let cell = Arc::new(OnceCell::new());
    let mut handles = Vec::new();
    
    for _ in 0..10 {
        let cell_cloned = Arc::clone(&cell);
        handles.push(thread::spawn(move || {
            cell_cloned.get_or_init(|| NonZeroUsize::new(42).unwrap())
        }));
    }

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    let expected_value = NonZeroUsize::new(42).unwrap();
    for result in results {
        assert_eq!(*result, expected_value);
    }
    assert_eq!(cell.get().unwrap(), &expected_value);
}

