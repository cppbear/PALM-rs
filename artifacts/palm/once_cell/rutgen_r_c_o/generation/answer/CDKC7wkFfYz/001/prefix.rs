// Answer 0

#[test]
fn test_once_ref_new_empty() {
    let once_ref: OnceRef<u32> = OnceRef::new();
}

#[test]
fn test_once_ref_new_empty_with_i32() {
    let once_ref: OnceRef<i32> = OnceRef::new();
}

#[test]
fn test_once_ref_new_empty_with_str() {
    let once_ref: OnceRef<&str> = OnceRef::new();
}

#[test]
fn test_once_ref_new_empty_with_custom_struct() {
    struct CustomStruct;
    let once_ref: OnceRef<CustomStruct> = OnceRef::new();
}

#[test]
fn test_once_ref_new_empty_with_reference() {
    let value: i32 = 10;
    let once_ref: OnceRef<&i32> = OnceRef::new();
}

