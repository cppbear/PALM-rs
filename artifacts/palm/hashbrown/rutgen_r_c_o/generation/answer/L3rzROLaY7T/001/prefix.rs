// Answer 0

#[test]
fn test_remove_entry_with_valid_key() {
    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Occupied(o) => {
            let entry = o.remove_entry();
        }
        _ => panic!(),
    }
}

#[test]
fn test_remove_entry_with_another_valid_key() {
    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    match map.raw_entry_mut().from_key(&"b") {
        RawEntryMut::Occupied(o) => {
            let entry = o.remove_entry();
        }
        _ => panic!(),
    }
}

#[test]
fn test_remove_entry_with_non_existing_key() {
    let mut map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    match map.raw_entry_mut().from_key(&"c") {
        RawEntryMut::Vacant(_) => {}
        _ => panic!(),
    }
}

#[test]
fn test_remove_entry_from_empty_table() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Vacant(_) => {}
        _ => panic!(),
    }
}

#[test]
fn test_remove_entry_edge_case_empty_value() {
    let mut map: HashMap<&str, Option<u32>> = [("a", None), ("b", Some(200))].into();
    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Occupied(o) => {
            let entry = o.remove_entry();
        }
        _ => panic!(),
    }
}

#[test]
fn test_remove_entry_edge_case_negative_value() {
    let mut map: HashMap<&str, i32> = [("a", -100), ("b", 200)].into();
    match map.raw_entry_mut().from_key(&"a") {
        RawEntryMut::Occupied(o) => {
            let entry = o.remove_entry();
        }
        _ => panic!(),
    }
}

