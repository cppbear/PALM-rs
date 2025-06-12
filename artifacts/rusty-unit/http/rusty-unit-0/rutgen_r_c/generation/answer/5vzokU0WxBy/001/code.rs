// Answer 0

#[test]
fn test_try_insert_entry() {
    struct DummyHeaderValue;

    let mut map = HeaderMap::with_capacity(1);
    let key = HeaderName { inner: Default::default() };
    
    let vacant_entry = VacantEntry {
        map: &mut map,
        key,
        hash: HashValue(0),
        probe: 0,
        danger: false,
    };

    // Trigger successful insertion (this should succeed without panic)
    let inserted_entry = vacant_entry.try_insert_entry(DummyHeaderValue).unwrap();
    assert_eq!(map.len(), 1);
    
    // Trigger MaxSizeReached error
    let mut map_full = HeaderMap::with_capacity(1);
    let full_key = HeaderName { inner: Default::default() };
    
    let full_vacant_entry = VacantEntry {
        map: &mut map_full,
        key: full_key,
        hash: HashValue(1),
        probe: 0,
        danger: false,
    };
    
    // Insert an entry to make the map full
    full_vacant_entry.try_insert_entry(DummyHeaderValue).unwrap();

    // Attempt to insert another entry which should fail
    let result = full_vacant_entry.try_insert_entry(DummyHeaderValue);
    assert!(result.is_err());
}

#[test]
fn test_try_insert_entry_boundary_conditions() {
    struct BoundaryHeaderValue;

    let mut map = HeaderMap::with_capacity(0); // start with 0 capacity
    let key = HeaderName { inner: Default::default() };

    let vacant_entry = VacantEntry {
        map: &mut map,
        key,
        hash: HashValue(0),
        probe: 0,
        danger: false,
    };

    // Check that inserting into empty map fails due to MaxSizeReached
    let result = vacant_entry.try_insert_entry(BoundaryHeaderValue);
    assert!(result.is_err());

    // Ensure that the map remains empty
    assert_eq!(map.len(), 0);
}

