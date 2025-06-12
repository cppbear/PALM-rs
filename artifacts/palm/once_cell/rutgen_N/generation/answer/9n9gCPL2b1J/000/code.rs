// Answer 0

#[derive(Debug)]
struct Inner<T> {
    inner: std::sync::atomic::AtomicPtr<T>,
}

impl<T> Inner<T> {
    pub fn new(value: T) -> Self {
        let box_value = Box::new(value);
        let ptr = Box::into_raw(box_value);
        Inner {
            inner: std::sync::atomic::AtomicPtr::new(ptr),
        }
    }
    
    pub fn get(&self) -> Option<&T> {
        let ptr = self.inner.load(std::sync::atomic::Ordering::Acquire);
        unsafe { ptr.as_ref() }
    }
}

#[test]
fn test_get_some() {
    let inner = Inner::new(42);
    assert_eq!(inner.get(), Some(&42));
}

#[test]
fn test_get_none() {
    let inner = Inner::<i32>::new(0);
    let ptr = inner.inner.load(std::sync::atomic::Ordering::Acquire);
    unsafe {
        std::ptr::drop_in_place(ptr); // Manually drop the value to simulate 'None'
    }
    assert_eq!(inner.get(), None);
}

