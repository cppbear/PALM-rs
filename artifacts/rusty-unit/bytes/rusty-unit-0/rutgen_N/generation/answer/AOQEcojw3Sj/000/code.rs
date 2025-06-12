// Answer 0

#[test]
fn test_owned_box_and_drop() {
    struct Owned<T> {
        value: T,
    }

    unsafe fn create_and_drop() {
        let owned = Owned { value: 42 };
        let ptr = Box::into_raw(Box::new(owned)) as *mut ();
        owned_box_and_drop::<i32>(ptr);
    }

    create_and_drop();
}

#[test]
#[should_panic]
fn test_owned_box_and_drop_null_pointer() {
    unsafe {
        owned_box_and_drop::<i32>(std::ptr::null_mut());
    }
}

