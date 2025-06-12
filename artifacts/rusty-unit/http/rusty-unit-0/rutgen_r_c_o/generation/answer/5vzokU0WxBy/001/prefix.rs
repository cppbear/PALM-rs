// Answer 0

#[test]
fn test_try_insert_entry_success() {
    let mut map = HeaderMap::with_capacity(1024);
    let key = HeaderName { inner: Repr::from("test-header") };
    let value = HeaderValue::from("test-value");
    let probe = 0;
    let danger = false;
    let hash = HashValue(12345);
    
    let vacant_entry = VacantEntry {
        map: &mut map,
        key,
        hash,
        probe,
        danger,
    };
    
    let occupied_entry = vacant_entry.try_insert_entry(value).unwrap();
}

#[test]
fn test_try_insert_entry_max_size_reached() {
    let capacity = 1;
    let mut map = HeaderMap::with_capacity(capacity);
    let key = HeaderName { inner: Repr::from("header-full") };
    let value_one = HeaderValue::from("value-1");
    let value_two = HeaderValue::from("value-2");
    
    let probe = 0;
    let danger = false;
    let hash = HashValue(54321);
    
    let vacant_entry = VacantEntry {
        map: &mut map,
        key,
        hash,
        probe,
        danger,
    };
    
    let _ = vacant_entry.try_insert_entry(value_one).unwrap();
    let occupied_entry_two = vacant_entry.try_insert_entry(value_two);
    
    assert!(occupied_entry_two.is_err());
}

#[test]
#[should_panic]
fn test_try_insert_entry_invalid_probe() {
    let mut map = HeaderMap::with_capacity(512);
    let key = HeaderName { inner: Repr::from("invalid-probe") };
    let value = HeaderValue::from("value");
    let probe = 512; // Invalid probe
    let danger = false;
    let hash = HashValue(67890);
    
    let vacant_entry = VacantEntry {
        map: &mut map,
        key,
        hash,
        probe,
        danger,
    };

    let _ = vacant_entry.try_insert_entry(value);
}

#[test]
fn test_try_insert_entry_with_danger() {
    let mut map = HeaderMap::with_capacity(2048);
    let key = HeaderName { inner: Repr::from("danger-insert") };
    let value = HeaderValue::from("danger-value");
    let probe = 1; // Arbitrary valid probe
    let danger = true; // Danger set to true
    let hash = HashValue(11111);
    
    let vacant_entry = VacantEntry {
        map: &mut map,
        key,
        hash,
        probe,
        danger,
    };
    
    let occupied_entry = vacant_entry.try_insert_entry(value).unwrap();
}

