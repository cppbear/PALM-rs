// Answer 0

#[test]
fn test_get_mut_valid() {
    let mut map = HeaderMap::default();
    map.entries.push(Bucket {
        hash: 1,
        key: HeaderName::from("host").unwrap(),
        value: "hello.world".to_string(),
        links: None,
    });
    let mut entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0,
    };
    let value_mut = entry.get_mut();
    value_mut.push_str("-test");
}

#[test]
#[should_panic]
fn test_get_mut_empty_entry() {
    let mut map = HeaderMap::default();
    let mut entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 0, // this index is valid but should panic since entries are empty
    };
    let _ = entry.get_mut();
}

#[test]
fn test_get_mut_multiple_entries() {
    let mut map = HeaderMap::default();
    map.entries.push(Bucket {
        hash: 1,
        key: HeaderName::from("host").unwrap(),
        value: "hello.world".to_string(),
        links: None,
    });
    map.entries.push(Bucket {
        hash: 2,
        key: HeaderName::from("user-agent").unwrap(),
        value: "test-agent".to_string(),
        links: None,
    });
    
    let mut entry = OccupiedEntry {
        map: &mut map,
        probe: 1,
        index: 1,
    };
    let value_mut = entry.get_mut();
    value_mut.push_str("-modified");
}

#[test]
#[should_panic]
fn test_get_mut_invalid_index() {
    let mut map = HeaderMap::default();
    map.entries.push(Bucket {
        hash: 1,
        key: HeaderName::from("host").unwrap(),
        value: "hello.world".to_string(),
        links: None,
    });
    
    let mut entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 1, // invalid index
    };
    let _ = entry.get_mut();
}

#[test]
fn test_get_mut_max_index() {
    let mut map = HeaderMap::default();
    for i in 0..65535 {
        map.entries.push(Bucket {
            hash: i as u64,
            key: HeaderName::from(format!("key{}", i)).unwrap(),
            value: format!("value{}", i),
            links: None,
        });
    }
    
    let mut entry = OccupiedEntry {
        map: &mut map,
        probe: 0,
        index: 65534, // maximum valid index
    };
    let value_mut = entry.get_mut();
    value_mut.push_str("-updated");
}

