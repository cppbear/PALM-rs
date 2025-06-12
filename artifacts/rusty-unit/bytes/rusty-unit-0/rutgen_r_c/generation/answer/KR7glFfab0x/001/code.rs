// Answer 0

#[test]
fn test_vtable_debug_struct() {
    struct TestVtable {
        clone: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> Bytes,
        drop: unsafe fn(&mut AtomicPtr<()>, *const u8, usize),
    }

    const TEST_VTABLE: TestVtable = TestVtable {
        clone: core::ptr::null_mut(),
        drop: core::ptr::null_mut(),
    };
    
    let vtable_ptr = &TEST_VTABLE as *const TestVtable;
    
    let result = {
        let f = &mut fmt::Formatter::new();
        let vtable = Vtable {
            clone: TEST_VTABLE.clone,
            drop: TEST_VTABLE.drop,
            into_vec: core::ptr::null_mut(),
            into_mut: core::ptr::null_mut(),
            is_unique: core::ptr::null_mut(),
        };
        vtable.fmt(f).is_ok()
    };

    assert!(result);
}

#[test]
fn test_vtable_debug_struct_with_null() {
    struct TestVtable {
        clone: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> Bytes,
        drop: unsafe fn(&mut AtomicPtr<()>, *const u8, usize),
    }

    const TEST_VTABLE: TestVtable = TestVtable {
        clone: core::ptr::null_mut(),
        drop: core::ptr::null_mut(),
    };
    
    let result = {
        let f = &mut fmt::Formatter::new();
        let vtable = Vtable {
            clone: TEST_VTABLE.clone,
            drop: TEST_VTABLE.drop,
            into_vec: core::ptr::null_mut(),
            into_mut: core::ptr::null_mut(),
            is_unique: core::ptr::null_mut(),
        };
        vtable.fmt(f).is_ok()
    };

    assert!(result);
}

#[test]
#[should_panic]
fn test_vtable_debug_struct_with_invalid_ptr() {
    struct TestVtable {
        clone: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> Bytes,
        drop: unsafe fn(&mut AtomicPtr<()>, *const u8, usize),
    }

    let invalid_clone: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> Bytes = 
        unsafe { std::mem::transmute(1usize) };
    let invalid_drop: unsafe fn(&mut AtomicPtr<()>, *const u8, usize) = 
        unsafe { std::mem::transmute(1usize) };

    let vtable = Vtable {
        clone: invalid_clone,
        drop: invalid_drop,
        into_vec: core::ptr::null_mut(),
        into_mut: core::ptr::null_mut(),
        is_unique: core::ptr::null_mut(),
    };
    
    let f = &mut fmt::Formatter::new();
    vtable.fmt(f).unwrap();
}

