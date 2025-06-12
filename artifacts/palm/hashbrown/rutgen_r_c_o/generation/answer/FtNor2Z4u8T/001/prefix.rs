// Answer 0

#[test]
fn test_get_existing_entry_poneyland() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 12);
    let entry = map.entry("poneyland").or_insert(0);
    *entry = 12;
    
    match map.entry("poneyland") {
        Entry::Occupied(entry) => {
            let value = entry.get();
        }
        Entry::Vacant(_) => panic!(),
    }
}

#[test]
fn test_get_existing_entry_unicornland() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("unicornland", 25);
    let entry = map.entry("unicornland").or_insert(0);
    *entry = 25;
    
    match map.entry("unicornland") {
        Entry::Occupied(entry) => {
            let value = entry.get();
        }
        Entry::Vacant(_) => panic!(),
    }
}

#[test]
fn test_get_with_multiple_entries() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 12);
    map.insert("unicornland", 25);
    
    match map.entry("poneyland") {
        Entry::Occupied(entry) => {
            let value = entry.get();
        }
        Entry::Vacant(_) => panic!(),
    }

    match map.entry("unicornland") {
        Entry::Occupied(entry) => {
            let value = entry.get();
        }
        Entry::Vacant(_) => panic!(),
    }
}

#[test]
fn test_get_edge_case_single_entry() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.insert("poneyland", 0);
    
    match map.entry("poneyland") {
        Entry::Occupied(entry) => {
            let value = entry.get();
        }
        Entry::Vacant(_) => panic!(),
    }
}

#[test]
fn test_get_with_large_map() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    for i in 0..100 {
        map.insert(format!("key{}", i).as_str(), i as u32);
    }
    
    match map.entry("key99") {
        Entry::Occupied(entry) => {
            let value = entry.get();
        }
        Entry::Vacant(_) => panic!(),
    }
}

