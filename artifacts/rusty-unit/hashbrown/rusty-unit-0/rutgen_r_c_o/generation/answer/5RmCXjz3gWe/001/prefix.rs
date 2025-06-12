// Answer 0

#[test]
fn test_remove_entry_existing() {
    let mut set: HashSet<&str> = HashSet::new();
    set.entry("item1").or_insert();
    
    if let Entry::Occupied(o) = set.entry("item1") {
        let _ = o.remove();
    }
}

#[test]
fn test_remove_entry_empty() {
    let mut set: HashSet<&str> = HashSet::new();
    
    if let Entry::Occupied(o) = set.entry("item2") {
        let _ = o.remove();
    }
}

#[test]
fn test_remove_entry_capacity() {
    let mut set: HashSet<&str> = HashSet::with_capacity(10);
    set.entry("item3").or_insert();
    let capacity_before_remove = set.capacity();
    
    if let Entry::Occupied(o) = set.entry("item3") {
        let _ = o.remove();
    }
    assert_eq!(set.len(), 0);
    assert_eq!(set.capacity(), capacity_before_remove);
}

#[test]
#[should_panic]
fn test_remove_non_existent() {
    let mut set: HashSet<&str> = HashSet::new();
    
    if let Entry::Occupied(o) = set.entry("non_existent") {
        let _ = o.remove();
    }
}

#[test]
fn test_remove_entry_multiple_elements() {
    let mut set: HashSet<&str> = HashSet::new();
    set.entry("item4").or_insert();
    set.entry("item5").or_insert();
    
    if let Entry::Occupied(o) = set.entry("item4") {
        let _ = o.remove();
    }
    assert_eq!(set.len(), 1);
}

