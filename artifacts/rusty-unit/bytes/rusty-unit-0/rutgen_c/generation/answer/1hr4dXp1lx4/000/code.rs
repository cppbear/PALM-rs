// Answer 0

#[test]
fn test_shallow_clone_arc() {
    let mut bytes_mut = unsafe {
        let mut instance = BytesMut::with_capacity(10);
        instance.set_len(5);
        instance.data = &mut 0 as *mut _ as *mut Shared;
        instance
    };

    let clone = unsafe { bytes_mut.shallow_clone() };

    assert_ne!(bytes_mut.data, clone.data);
}

#[test]
fn test_shallow_clone_vec() {
    let mut bytes_mut = unsafe {
        let mut instance = BytesMut::with_capacity(10);
        instance.set_len(5);
        instance.data = &mut 1 as *mut _ as *mut Shared;
        instance
    };

    let clone = unsafe { bytes_mut.shallow_clone() };

    assert_ne!(bytes_mut.data, clone.data);
}

#[test]
#[should_panic]
fn test_shallow_clone_invalid_state() {
    let mut bytes_mut = unsafe {
        let mut instance = BytesMut::with_capacity(10);
        instance.set_len(0);
        instance.data = ptr::null_mut();
        instance
    };

    unsafe {
        bytes_mut.shallow_clone();
    }
}

