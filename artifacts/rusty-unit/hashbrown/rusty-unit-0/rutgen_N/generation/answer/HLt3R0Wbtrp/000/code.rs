// Answer 0

#[derive(Debug)]
struct ScopeGuard<T> {
    value: T,
}

impl<T> ScopeGuard<T> {
    fn new(value: T) -> Self {
        ScopeGuard { value }
    }
    
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

#[test]
fn test_deref_mut() {
    let mut guard = ScopeGuard::new(5);
    let value: &mut i32 = guard.deref_mut();
    *value += 1;
    assert_eq!(*value, 6);
}

#[test]
fn test_deref_mut_boundary() {
    let mut guard = ScopeGuard::new(0);
    let value: &mut i32 = guard.deref_mut();
    *value = 100;
    assert_eq!(*value, 100);
}

