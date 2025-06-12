// Answer 0

#[test]
fn test_get_none() {
    let once = OnceNonZeroUsize::new();
    assert_eq!(once.get(), None);
}

#[test]
fn test_get_some() {
    let once = OnceNonZeroUsize::new();
    let non_zero_value = NonZeroUsize::new(5).unwrap();
    unsafe {
        once.inner.store(non_zero_value.get(), Ordering::Release);
    }
    assert_eq!(once.get(), Some(non_zero_value));
}

#[test]
fn test_get_with_zero() {
    let once = OnceNonZeroUsize::new();
    unsafe {
        once.inner.store(0, Ordering::Release);
    }
    assert_eq!(once.get(), None);
}

#[test]
fn test_get_multiple_threads() {
    use std::sync::Arc;
    use std::thread;

    let once = Arc::new(OnceNonZeroUsize::new());
    let once_clone = Arc::clone(&once);
    
    let non_zero_value = NonZeroUsize::new(10).unwrap();
    unsafe {
        once_clone.inner.store(non_zero_value.get(), Ordering::Release);
    }
    
    let handle = thread::spawn(move || {
        assert_eq!(once.get(), Some(non_zero_value));
    });
    
    handle.join().unwrap();
}

