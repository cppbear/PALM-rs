unsafe fn owned_box_and_drop<T>(ptr: *mut ()) {
    let b: Box<Owned<T>> = Box::from_raw(ptr as _);
    drop(b);
}