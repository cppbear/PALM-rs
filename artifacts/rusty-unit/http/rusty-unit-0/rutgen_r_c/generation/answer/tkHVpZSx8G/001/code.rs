// Answer 0

#[test]
fn test_or_insert_with_occupied_entry() {
    struct TestHeaderMap {
        map: http::HeaderMap<u32>,
    }

    let mut header_map = TestHeaderMap {
        map: http::HeaderMap::default(),
    };

    let header_name = http::HeaderName::from_static("x-occupied");
    header_map.map.insert(header_name.clone(), 1);

    let entry = header_map.map.entry(header_name).or_insert(0);
    *entry += 1;

    assert_eq!(header_map.map[header_name], 2);
}

#[test]
fn test_or_insert_with_vacant_entry() {
    struct TestHeaderMap {
        map: http::HeaderMap<u32>,
    }

    let mut header_map = TestHeaderMap {
        map: http::HeaderMap::default(),
    };

    let header_name = http::HeaderName::from_static("x-vacant");

    let entry = header_map.map.entry(header_name).or_insert(5);
    assert_eq!(*entry, 5);
}

#[test]
#[should_panic(expected = "size overflows MAX_SIZE")] 
fn test_or_insert_panics_on_exceeding_max_size() {
    struct TestHeaderMap {
        map: http::HeaderMap<u32>,
    }

    const MAX_ENTRIES: usize = 1 << 15; // 32768
    let mut header_map = TestHeaderMap {
        map: http::HeaderMap::default(),
    };

    for i in 0..MAX_ENTRIES {
        let header_name = http::HeaderName::from_static(&format!("x-overflow-{}", i));
        header_map.map.entry(header_name).or_insert(i as u32);
    }
}

