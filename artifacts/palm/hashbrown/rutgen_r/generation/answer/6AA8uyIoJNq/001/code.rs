// Answer 0

#[test]
fn test_guard_with_integer_value() {
    let mut value = 10;
    let dropfn = |v: &mut i32| {
        *v += 5;
    };
    let guard_instance = guard(value, dropfn);
    
    assert_eq!(guard_instance.value, 10);
}

#[test]
fn test_guard_with_string_value() {
    let value = String::from("hello");
    let dropfn = |v: &mut String| {
        v.push_str(" world");
    };
    let mut guard_instance = guard(value, dropfn);
    dropfn(&mut guard_instance.value);
    
    assert_eq!(guard_instance.value, "hello world");
}

#[test]
#[should_panic]
fn test_guard_with_panic() {
    let value = vec![1, 2, 3];
    let dropfn = |v: &mut Vec<i32>| {
        panic!("Intentional panic");
    };
    let guard_instance = guard(value, dropfn);
    
    dropfn(&mut guard_instance.value);
}

#[test]
fn test_guard_with_float_value() {
    let mut value = 3.14;
    let dropfn = |v: &mut f64| {
        *v *= 2.0;
    };
    let guard_instance = guard(value, dropfn);
    assert_eq!(guard_instance.value, 3.14);
    
    dropfn(&mut guard_instance.value);
    assert_eq!(guard_instance.value, 6.28);
}

