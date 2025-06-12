// Answer 0

#[test]
fn test_append_single_value() {
    let mut map = HeaderMap::new();
    map.insert("host".parse().unwrap(), "world".parse().unwrap());

    if let OccupiedEntry { index: idx, .. } = map.entry("host").unwrap() {
        map.entries.push(Bucket {
            hash: 123.into(),
            key: "host".parse().unwrap(),
            value: "world".parse().unwrap(),
            links: None,
        });
        let mut occupied_entry = OccupiedEntry {
            map: &mut map,
            probe: 0,
            index: idx,
        };
        occupied_entry.append("earth".parse().unwrap());
    }
}

#[test]
fn test_append_multiple_values() {
    let mut map = HeaderMap::new();
    map.insert("host".parse().unwrap(), "world".parse().unwrap());
    map.insert("host".parse().unwrap(), "universe".parse().unwrap());

    if let OccupiedEntry { index: idx, .. } = map.entry("host").unwrap() {
        map.entries.push(Bucket {
            hash: 456.into(),
            key: "host".parse().unwrap(),
            value: "world".parse().unwrap(),
            links: None,
        });
        let mut occupied_entry = OccupiedEntry {
            map: &mut map,
            probe: 0,
            index: idx,
        };
        occupied_entry.append("earth".parse().unwrap());
        occupied_entry.append("galaxy".parse().unwrap());
    }
}

#[test]
fn test_append_edge_case_empty_value() {
    let mut map = HeaderMap::new();
    map.insert("host".parse().unwrap(), "world".parse().unwrap());

    if let OccupiedEntry { index: idx, .. } = map.entry("host").unwrap() {
        map.entries.push(Bucket {
            hash: 789.into(),
            key: "host".parse().unwrap(),
            value: "world".parse().unwrap(),
            links: None,
        });
        let mut occupied_entry = OccupiedEntry {
            map: &mut map,
            probe: 0,
            index: idx,
        };
        occupied_entry.append("".parse().unwrap());
    }
}

#[test]
fn test_append_large_value() {
    let mut map = HeaderMap::new();
    map.insert("host".parse().unwrap(), "world".parse().unwrap());

    if let OccupiedEntry { index: idx, .. } = map.entry("host").unwrap() {
        map.entries.push(Bucket {
            hash: 321.into(),
            key: "host".parse().unwrap(),
            value: "world".parse().unwrap(),
            links: None,
        });
        let mut occupied_entry = OccupiedEntry {
            map: &mut map,
            probe: 0,
            index: idx,
        };
        occupied_entry.append("a".repeat(255).parse().unwrap());
    }
}

