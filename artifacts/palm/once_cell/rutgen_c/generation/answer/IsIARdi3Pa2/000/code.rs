// Answer 0

#[test]
fn test_once_non_zero_usize_new() {
    let instance = OnceNonZeroUsize::new();
    assert_eq!(instance.inner.load(Ordering::SeqCst), 0);
}

#[test]
fn test_once_non_zero_usize_set_and_get() {
    let instance = OnceNonZeroUsize::new();
    let value = NonZeroUsize::new(5).unwrap();
    assert_eq!(instance.set(value), Ok(()));
    assert_eq!(instance.inner.load(Ordering::SeqCst), 5);
}

#[test]
fn test_once_non_zero_usize_get() {
    let instance = OnceNonZeroUsize::new();
    let value = NonZeroUsize::new(10).unwrap();
    instance.set(value).unwrap();
    assert_eq!(instance.get(), Some(value));
}

#[test]
fn test_once_non_zero_usize_get_unchecked() {
    let instance = OnceNonZeroUsize::new();
    let value = NonZeroUsize::new(15).unwrap();
    instance.set(value).unwrap();
    unsafe {
        assert_eq!(instance.get_unchecked(), value);
    }
}

#[test]
fn test_once_non_zero_usize_get_or_init() {
    let instance = OnceNonZeroUsize::new();
    let result = instance.get_or_init(|| NonZeroUsize::new(20).unwrap());
    assert_eq!(result, NonZeroUsize::new(20).unwrap());
    assert_eq!(instance.inner.load(Ordering::SeqCst), 20);
}

#[test]
fn test_once_non_zero_usize_get_or_try_init() {
    let instance = OnceNonZeroUsize::new();
    let result: Result<NonZeroUsize, ()> = instance.get_or_try_init(|| Ok(NonZeroUsize::new(25).unwrap()));
    assert_eq!(result, Ok(NonZeroUsize::new(25).unwrap()));
    assert_eq!(instance.inner.load(Ordering::SeqCst), 25);
}

