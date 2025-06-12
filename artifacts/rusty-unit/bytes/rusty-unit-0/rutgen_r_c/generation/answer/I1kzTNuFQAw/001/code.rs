// Answer 0

#[test]
fn test_with_mut_non_null_pointer() {
    let mut value: i32 = 42;
    let ptr = AtomicPtr::new(&mut value);
    
    ptr.with_mut(|p| {
        **p += 1;
    });

    assert_eq!(unsafe { *ptr.load(Ordering::SeqCst) }, 43);
}

#[test]
fn test_with_mut_null_pointer() {
    let mut ptr: AtomicPtr<i32> = AtomicPtr::new(std::ptr::null_mut());

    let result = std::panic::catch_unwind(|| {
        ptr.with_mut(|p| {
            assert!(p.is_null());
        });
    });

    assert!(result.is_err());
}

#[test]
fn test_with_mut_modify_value() {
    let mut value: f64 = 3.14;
    let ptr = AtomicPtr::new(&mut value);
    
    ptr.with_mut(|p| {
        **p = 2.71;
    });
    
    assert_eq!(unsafe { *ptr.load(Ordering::SeqCst) }, 2.71);
}

#[test]
fn test_with_mut_multiple_modifications() {
    let mut value: u64 = 100;
    let ptr = AtomicPtr::new(&mut value);
    
    for _ in 0..5 {
        ptr.with_mut(|p| {
            **p += 10;
        });
    }
    
    assert_eq!(unsafe { *ptr.load(Ordering::SeqCst) }, 150);
}

