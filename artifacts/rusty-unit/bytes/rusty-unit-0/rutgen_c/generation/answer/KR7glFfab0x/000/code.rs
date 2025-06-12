// Answer 0

#[test]
fn test_vtable_debug_fmt() {
    struct VtableTest {
        clone: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> Bytes,
        drop: unsafe fn(&mut AtomicPtr<()>, *const u8, usize),
    }

    let vtable = Vtable {
        clone: VtableTest::clone,
        drop: VtableTest::drop,
        into_vec: owned_to_vec,
        into_mut: owned_to_mut,
        is_unique: owned_is_unique,
    };

    let debug_output = format!("{:?}", vtable);
    assert!(debug_output.contains("clone"));
    assert!(debug_output.contains("drop"));
}

#[test]
fn test_vtable_debug_fmt_with_null() {
    struct VtableTest {
        clone: unsafe fn(&AtomicPtr<()>, *const u8, usize) -> Bytes,
        drop: unsafe fn(&mut AtomicPtr<()>, *const u8, usize),
    }

    let vtable = Vtable {
        clone: VtableTest::clone,
        drop: VtableTest::drop,
        into_vec: owned_to_vec,
        into_mut: owned_to_mut,
        is_unique: owned_is_unique,
    };

    let debug_output = format!("{:?}", vtable);
    assert_eq!(debug_output.len(), vtable.debug_struct.len() + 2); // check for extra '}' that represents debug output closure
}

