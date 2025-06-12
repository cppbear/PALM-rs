// Answer 0

#[test]
fn test_or_try_insert_with_vacant_entry() {
    use http::HeaderMap;

    let mut map = HeaderMap::new();
    
    // Test inserting a new value using default function when the entry is vacant
    let res = map.try_entry("x-hello")
        .unwrap()
        .or_try_insert_with(|| "world".parse().unwrap())
        .unwrap();

    assert_eq!(res, "world");
}

#[test]
fn test_or_try_insert_with_vacant_entry_multiple_inserts() {
    use http::HeaderMap;

    let mut map = HeaderMap::new();
    
    // Insert first value
    let res1 = map.try_entry("x-hello")
        .unwrap()
        .or_try_insert_with(|| "first".parse().unwrap())
        .unwrap();
        
    assert_eq!(res1, "first");
    
    // Attempt to insert another value, proving that the first value persists
    let res2 = map.try_entry("x-hello")
        .unwrap()
        .or_try_insert_with(|| "second".parse().unwrap())
        .unwrap();
        
    assert_eq!(res2, "first"); // Should still be "first", not "second"
}

#[test]
#[should_panic]
fn test_or_try_insert_with_fails_on_unreachable() {
    use http::HeaderMap;
    use std::panic;

    let mut map = HeaderMap::new();
    
    // This will panic because we are trying to access an entry that hasn't been inserted.
    let _ = panic::catch_unwind(|| {
        map.try_entry("non-existent")
            .unwrap()
            .or_try_insert_with(|| unreachable!());
    });

    // Ensure insertion does not happen, and thus no entry exists
    assert!(map.get("non-existent").is_none());
}

