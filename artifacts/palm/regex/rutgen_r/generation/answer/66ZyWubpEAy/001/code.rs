// Answer 0

#[test]
fn test_swap_greed_none() {
    struct TestStruct {
        swap_greed: Option<bool>,
    }
    
    let instance = TestStruct { swap_greed: None };
    assert_eq!(instance.swap_greed(), false);
}

#[test]
fn test_swap_greed_some_true() {
    struct TestStruct {
        swap_greed: Option<bool>,
    }
    
    let instance = TestStruct { swap_greed: Some(true) };
    assert_eq!(instance.swap_greed(), true);
}

#[test]
fn test_swap_greed_some_false() {
    struct TestStruct {
        swap_greed: Option<bool>,
    }
    
    let instance = TestStruct { swap_greed: Some(false) };
    assert_eq!(instance.swap_greed(), false);
}

