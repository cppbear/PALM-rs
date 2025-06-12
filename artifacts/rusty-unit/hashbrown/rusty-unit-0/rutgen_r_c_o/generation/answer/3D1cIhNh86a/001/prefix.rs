// Answer 0

#[test]
fn test_get_mut_valid_case() {
    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(1, 500);
    if let OccupiedEntry { elem, .. } = map.entry(1) {
        let mut entry = OccupiedEntry { elem, table: &mut map };
        let value = entry.get_mut();
        *value += 100;
    }
}

#[test]
fn test_get_mut_multiple_updates() {
    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(2, 300);
    if let OccupiedEntry { elem, .. } = map.entry(2) {
        let mut entry = OccupiedEntry { elem, table: &mut map };
        let value = entry.get_mut();
        *value += 50;
        *value += 20;
    }
}

#[test]
fn test_get_mut_edge_case_first_entry() {
    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(1, 1);
    if let OccupiedEntry { elem, .. } = map.entry(1) {
        let mut entry = OccupiedEntry { elem, table: &mut map };
        let value = entry.get_mut();
        *value += 1;
    }
}

#[test]
fn test_get_mut_edge_case_max_value() {
    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(3, 1000000);
    if let OccupiedEntry { elem, .. } = map.entry(3) {
        let mut entry = OccupiedEntry { elem, table: &mut map };
        let value = entry.get_mut();
        *value += 1;
    }
}

#[test]
fn test_get_mut_with_different_keys() {
    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(4, 250);
    map.insert(5, 500);
    
    if let OccupiedEntry { elem, .. } = map.entry(4) {
        let mut entry = OccupiedEntry { elem, table: &mut map };
        let value = entry.get_mut();
        *value += 100;
    }
    
    if let OccupiedEntry { elem, .. } = map.entry(5) {
        let mut entry = OccupiedEntry { elem, table: &mut map };
        let value = entry.get_mut();
        *value += 200;
    }
}

