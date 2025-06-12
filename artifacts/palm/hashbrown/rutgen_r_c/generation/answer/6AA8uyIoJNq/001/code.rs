// Answer 0

#[test]
fn test_guard_with_simple_value() {
    let mut value = 42;
    let drop_fn = |v: &mut i32| *v += 1;
    let guard_instance = guard(value, drop_fn);
    
    let mut value_ref = guard_instance.value; // Accessing the value
    assert_eq!(value_ref, 42);
}

#[test]
fn test_guard_with_string() {
    let value = String::from("Hello");
    let mut dropped = false;
    let drop_fn = |v: &mut String| {
        *v = String::from("Dropped");
        dropped = true;
    };
    let mut guard_instance = guard(value, drop_fn);
    
    assert_eq!(guard_instance.value, "Hello");
    drop(guard_instance); // Dropping the guard to invoke the drop function
    assert!(dropped);
}

#[test]
fn test_guard_with_struct() {
    struct MyStruct {
        data: i32,
    }
    
    let mut value = MyStruct { data: 10 };
    let drop_fn = |v: &mut MyStruct| v.data *= 2;
    let guard_instance = guard(value, drop_fn);
    
    assert_eq!(guard_instance.value.data, 10);
    
    drop(guard_instance); // Implicitly calls drop_fn
    assert_eq!(value.data, 20);
}

#[should_panic]
fn test_guard_with_panic_in_drop_fn() {
    let value = 100;
    let drop_fn = |_: &mut i32| panic!("This drop function panicked!");
    let _guard_instance = guard(value, drop_fn);
}

