// Answer 0

#[cfg(test)]
use std::ptr::NonNull;

struct RawTableInner<T> {
    ctrl: NonNull<T>,
}

impl<T> RawTableInner<T> {
    fn new(ctrl: NonNull<T>) -> Self {
        RawTableInner { ctrl }
    }

    fn data_end(&self) -> NonNull<T> {
        self.ctrl.cast()
    }
}

#[test]
fn test_data_end() {
    let value = 42;
    let nonnull_value = NonNull::new(&value as *const _ as *mut _).unwrap();
    let table = RawTableInner::new(nonnull_value);

    let end_pointer: NonNull<i32> = table.data_end();
    assert_eq!(end_pointer.as_ptr(), &value as *const _ as *mut _);
}

#[test]
fn test_data_end_boundary() {
    let value = 0;
    let nonnull_value = NonNull::new(&value as *const _ as *mut _).unwrap();
    let table = RawTableInner::new(nonnull_value);

    let end_pointer: NonNull<i32> = table.data_end();
    assert_eq!(end_pointer.as_ptr(), &value as *const _ as *mut _);
}

