// Answer 0

#[test]
fn test_remove_found_empty_indices() {
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(1);
    let result = std::panic::catch_unwind(|| {
        let _ = map.remove_found(0, 0);
    });
    assert!(result.is_err());
}

#[test]
fn test_remove_found_with_valid_entry() {
    struct ValidHeader(String);
    impl IntoHeaderName for ValidHeader {
        fn into_header_name(self) -> HeaderName {
            HeaderName::try_from(self.0).unwrap()
        }
    }
    
    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(2);
    map.insert(ValidHeader("Header1".to_string()), 100);
    map.insert(ValidHeader("Header2".to_string()), 200);

    let found = 0; // assume that we are removing the first entry
    let probe = { 
        let pos = desired_pos(map.mask, HashValue(100)); 
        pos as usize 
    };

    let entry = map.remove_found(probe, found);
    assert_eq!(entry.key, HeaderName::try_from("Header1").unwrap());
    assert_eq!(entry.value, 100);
}

#[test]
fn test_remove_found_after_swap_remove() {
    struct ValidHeader(String);
    impl IntoHeaderName for ValidHeader {
        fn into_header_name(self) -> HeaderName {
            HeaderName::try_from(self.0).unwrap()
        }
    }

    let mut map: HeaderMap<u32> = HeaderMap::with_capacity(3);
    map.insert(ValidHeader("Header1".to_string()), 100);
    map.insert(ValidHeader("Header2".to_string()), 200);
    map.insert(ValidHeader("Header3".to_string()), 300);

    let found = 1; // Removing the second entry
    let probe = { 
        let pos = desired_pos(map.mask, HashValue(200)); 
        pos as usize 
    };

    let entry = map.remove_found(probe, found);
    assert_eq!(entry.key, HeaderName::try_from("Header2").unwrap());
    assert_eq!(entry.value, 200);
}

