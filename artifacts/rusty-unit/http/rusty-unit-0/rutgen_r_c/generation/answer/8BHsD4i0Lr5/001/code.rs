// Answer 0

#[test]
fn test_clone_box_with_integer() {
    struct IntWrapper(i32);
    let original = IntWrapper(5);
    let cloned = original.clone_box();
    assert_eq!(original.0, (*(cloned.as_any().downcast_ref::<IntWrapper>().unwrap())).0);
}

#[test]
fn test_clone_box_with_string() {
    struct StringWrapper(String);
    let original = StringWrapper("test".to_string());
    let cloned = original.clone_box();
    assert_eq!(original.0, (*(cloned.as_any().downcast_ref::<StringWrapper>().unwrap())).0);
}

#[test]
fn test_clone_box_with_vec() {
    struct VecWrapper(Vec<i32>);
    let original = VecWrapper(vec![1, 2, 3]);
    let cloned = original.clone_box();
    assert_eq!(original.0, (*(cloned.as_any().downcast_ref::<VecWrapper>().unwrap())).0);
}

#[test]
#[should_panic]
fn test_clone_box_with_non_clonable() {
    struct NonCloneableWrapper(*const u8);  // Non-clonable type
    let original = NonCloneableWrapper(std::ptr::null());
    let _ = original.clone_box(); // This should panic because it can't clone
}

