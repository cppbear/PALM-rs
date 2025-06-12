// Answer 0

#[test]
fn test_next_value_seed_with_some_bool() {
    let content = Content::Bool(true);
    let mut map_access = FlatMapAccess {
        iter: vec![Some((Content::Bool(true), content))].into_iter(),
        pending_content: None,
        _marker: PhantomData,
    };
    // Force next_key_seed to set pending_content
    map_access.next_key_seed(PhantomData).unwrap();
    let _ = map_access.next_value_seed(PhantomData);
}

#[test]
fn test_next_value_seed_with_some_u8() {
    let content = Content::U8(100);
    let mut map_access = FlatMapAccess {
        iter: vec![Some((Content::U8(100), content))].into_iter(),
        pending_content: None,
        _marker: PhantomData,
    };
    map_access.next_key_seed(PhantomData).unwrap();
    let _ = map_access.next_value_seed(PhantomData);
}

#[test]
fn test_next_value_seed_with_some_u16() {
    let content = Content::U16(50);
    let mut map_access = FlatMapAccess {
        iter: vec![Some((Content::U16(50), content))].into_iter(),
        pending_content: None,
        _marker: PhantomData,
    };
    map_access.next_key_seed(PhantomData).unwrap();
    let _ = map_access.next_value_seed(PhantomData);
}

#[test]
fn test_next_value_seed_with_some_string() {
    let content = Content::String("Test".to_string());
    let mut map_access = FlatMapAccess {
        iter: vec![Some((Content::String("Test".to_string()), content))].into_iter(),
        pending_content: None,
        _marker: PhantomData,
    };
    map_access.next_key_seed(PhantomData).unwrap();
    let _ = map_access.next_value_seed(PhantomData);
}

#[test]
fn test_next_value_seed_with_none() {
    let mut map_access = FlatMapAccess {
        iter: vec![Some((Content::None, Content::None))].into_iter(),
        pending_content: None,
        _marker: PhantomData,
    };
    map_access.next_key_seed(PhantomData).unwrap();
    let result = map_access.next_value_seed(PhantomData);
    // we expect an error here
    assert!(result.is_err());
}

