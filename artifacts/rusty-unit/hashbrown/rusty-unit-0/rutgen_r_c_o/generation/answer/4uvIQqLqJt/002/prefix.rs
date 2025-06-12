// Answer 0

#[test]
fn test_entry_occupied_get() {
    let mut set: hashbrown::HashSet<&str> = hashbrown::HashSet::new();
    set.insert("poneyland");
    let entry = set.entry("poneyland");
    
    if let hashbrown::hash_set::Entry::Occupied(occupied_entry) = entry {
        occupied_entry.get();
    }
}

#[test]
fn test_entry_vacant_get() {
    let mut set: hashbrown::HashSet<&str> = hashbrown::HashSet::new();
    let entry = set.entry("horseland");
    
    if let hashbrown::hash_set::Entry::Vacant(vacant_entry) = entry {
        vacant_entry.get();
    }
}

#[test]
fn test_entry_occupied_get_after_removal() {
    let mut set: hashbrown::HashSet<&str> = hashbrown::HashSet::new();
    set.insert("poneyland");
    let occupied_entry = set.entry("poneyland").or_insert();
    occupied_entry.remove();
    
    let entry = set.entry("poneyland");
    if let hashbrown::hash_set::Entry::Vacant(vacant_entry) = entry {
        vacant_entry.get();
    }
}

