// Answer 0

#[test]
fn test_guard_with_integer() {
    let dropfn = |value: &mut i32| {
        *value += 1;
    };
    let scope_guard = guard(10, dropfn);
}

#[test]
fn test_guard_with_string() {
    let dropfn = |value: &mut String| {
        value.push_str(" world");
    };
    let mut str_value = String::from("hello");
    let scope_guard = guard(str_value, dropfn);
}

#[test]
fn test_guard_with_float() {
    let dropfn = |value: &mut f64| {
        *value *= 2.0;
    };
    let scope_guard = guard(3.14, dropfn);
}

#[test]
fn test_guard_with_struct() {
    struct MyStruct {
        value: i32,
    }
    
    let dropfn = |value: &mut MyStruct| {
        value.value *= 5;
    };
    
    let my_struct = MyStruct { value: 2 };
    let scope_guard = guard(my_struct, dropfn);
}

#[test]
fn test_guard_with_enum() {
    enum MyEnum {
        A(i32),
        B(f64),
    }

    let dropfn = |value: &mut MyEnum| {
        if let MyEnum::A(ref mut x) = value {
            *x += 10;
        }
    };

    let enum_value = MyEnum::A(1);
    let scope_guard = guard(enum_value, dropfn);
}

#[test]
fn test_guard_with_vector() {
    let dropfn = |value: &mut Vec<i32>| {
        value.push(42);
    };
    let vec_value = vec![1, 2, 3];
    let scope_guard = guard(vec_value, dropfn);
}

