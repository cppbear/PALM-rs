// Answer 0

#[test]
fn test_once_ref_new() {
    struct TestStruct;
    
    let once_ref: OnceRef<TestStruct> = OnceRef::new();
    assert_eq!(once_ref.inner.load(Ordering::Relaxed), ptr::null_mut());
}

#[test]
fn test_once_ref_set_and_get() {
    struct TestStruct;
    
    let once_ref: OnceRef<TestStruct> = OnceRef::new();
    let test_value = TestStruct;
    
    assert_eq!(once_ref.set(&test_value), Ok(()));
    assert_eq!(once_ref.get(), Some(&test_value));
}

#[test]
fn test_once_ref_get_or_init() {
    struct TestStruct;
    
    let once_ref: OnceRef<TestStruct> = OnceRef::new();
    
    let value = once_ref.get_or_init(|| &TestStruct);
    assert!(ptr::eq(value, &TestStruct));
}

#[test]
fn test_once_ref_get_or_try_init() {
    struct TestStruct;

    let once_ref: OnceRef<TestStruct> = OnceRef::new();
    
    let result = once_ref.get_or_try_init(|| Ok(&TestStruct));
    assert!(result.is_ok());
    assert!(ptr::eq(result.unwrap(), &TestStruct));
}

