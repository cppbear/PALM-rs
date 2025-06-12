// Answer 0

#[derive(Debug)]
struct ScopeGuard<T, F> {
    dropfn: F,
    value: T,
}

impl<T, F> Drop for ScopeGuard<T, F>
where
    F: FnMut(&mut T),
{
    fn drop(&mut self) {
        (self.dropfn)(&mut self.value);
    }
}

#[test]
fn test_guard_with_integer() {
    let mut value = 5;
    let mut dropped = false;

    let scope_guard = guard(value, |v| {
        *v += 10;
        dropped = true;
    });

    std::mem::drop(scope_guard);
    assert!(dropped);
    assert_eq!(value, 5); // Original value is left unchanged after drop
}

#[test]
fn test_guard_with_string() {
    let mut value = String::from("Hello");
    let mut dropped = false;

    let scope_guard = guard(value, |v| {
        v.push_str(", World!");
        dropped = true;
    });

    std::mem::drop(scope_guard);
    assert!(dropped);
    assert_eq!(value, "Hello"); // Original value is left unchanged after drop
}

#[test]
fn test_guard_with_empty_function() {
    let value = 10;
    let mut dropped = false;

    let scope_guard = guard(value, |_: &mut i32| {
        dropped = true; // No operation performed on value
    });

    std::mem::drop(scope_guard);
    assert!(dropped);
}

