// Answer 0

#[test]
fn test_extend_with_no_previous_values() {
    let mut ext_a = Extensions::new();
    let mut ext_b = Extensions::new();
    ext_a.extend(ext_b);
    assert!(ext_a.is_empty());
}

#[test]
fn test_extend_with_different_types() {
    let mut ext_a = Extensions::new();
    ext_a.insert(8u8);
    ext_a.insert(16u16);

    let mut ext_b = Extensions::new();
    ext_b.insert(4u8);
    ext_b.insert("hello");

    ext_a.extend(ext_b);
    assert_eq!(ext_a.len(), 3);
    assert_eq!(ext_a.get::<u8>(), Some(&4u8));
    assert_eq!(ext_a.get::<u16>(), Some(&16u16));
    assert_eq!(ext_a.get::<&'static str>().copied(), Some("hello"));
}

#[test]
fn test_extend_overwrites_existing_types() {
    let mut ext_a = Extensions::new();
    ext_a.insert(8u8);
    ext_a.insert(16u16);

    let mut ext_b = Extensions::new();
    ext_b.insert(4u8); // this should overwrite ext_a's u8
    ext_b.insert("hello");

    ext_a.extend(ext_b);
    assert_eq!(ext_a.len(), 3);
    assert_eq!(ext_a.get::<u8>(), Some(&4u8));
    assert_eq!(ext_a.get::<u16>(), Some(&16u16));
    assert_eq!(ext_a.get::<&'static str>().copied(), Some("hello"));
}

#[test]
fn test_extend_with_empty_other() {
    let mut ext_a = Extensions::new();
    ext_a.insert(8u8);

    let ext_b = Extensions::new();
    ext_a.extend(ext_b);
    assert_eq!(ext_a.len(), 1);
    assert_eq!(ext_a.get::<u8>(), Some(&8u8));
}

