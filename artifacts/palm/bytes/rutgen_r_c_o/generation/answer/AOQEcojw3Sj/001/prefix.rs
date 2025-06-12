// Answer 0

#[test]
#[should_panic]
fn test_owned_box_and_drop_null_pointer() {
    unsafe {
        owned_box_and_drop::<i32>(std::ptr::null_mut());
    }
}

#[test]
fn test_owned_box_and_drop_valid_pointer() {
    unsafe {
        let layout = Layout::new::<Owned<i32>>();
        let ptr = alloc::alloc::alloc(layout) as *mut Owned<i32>;
        if !ptr.is_null() {
            (*ptr) = Owned { lifetime: OwnedLifetime {}, owner: 42 };
            owned_box_and_drop::<i32>(ptr);
        }
        // No need to deallocate here, owned_box_and_drop will handle it
    }
}

#[test]
fn test_owned_box_and_drop_multiple_types() {
    unsafe {
        let layout = Layout::new::<Owned<String>>();
        let ptr = alloc::alloc::alloc(layout) as *mut Owned<String>;
        if !ptr.is_null() {
            (*ptr) = Owned { lifetime: OwnedLifetime {}, owner: String::from("Hello") };
            owned_box_and_drop::<String>(ptr);
        }
    }
}

#[test]
fn test_owned_box_and_drop_empty_string() {
    unsafe {
        let layout = Layout::new::<Owned<String>>();
        let ptr = alloc::alloc::alloc(layout) as *mut Owned<String>;
        if !ptr.is_null() {
            (*ptr) = Owned { lifetime: OwnedLifetime {}, owner: String::new() };
            owned_box_and_drop::<String>(ptr);
        }
    }
}

#[test]
fn test_owned_box_and_drop_with_different_data() {
    unsafe {
        let layout = Layout::new::<Owned<f64>>();
        let ptr = alloc::alloc::alloc(layout) as *mut Owned<f64>;
        if !ptr.is_null() {
            (*ptr) = Owned { lifetime: OwnedLifetime {}, owner: 3.14 };
            owned_box_and_drop::<f64>(ptr);
        }
    }
}

