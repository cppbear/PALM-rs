// Answer 0

#[test]
fn test_entry_vacant() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry::*;
    
    let mut set = HashSet::new();
    let value = 'a';
    
    match set.entry(value) {
        Vacant(entry) => {
            entry.insert();
            assert!(set.contains(&value));
        }
        _ => panic!("Expected Vacant entry"),
    }
}

#[test]
fn test_entry_occupied() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry::*;
    
    let mut set = HashSet::new();
    let value = 'b';
    
    set.insert(value);
    
    match set.entry(value) {
        Occupied(entry) => {
            entry.remove();
            assert!(!set.contains(&value));
        }
        _ => panic!("Expected Occupied entry"),
    }
}

#[test]
fn test_entry_mixed() {
    use hashbrown::HashSet;
    use hashbrown::hash_set::Entry::*;

    let mut singles = HashSet::new();
    let mut dupes = HashSet::new();
    
    for ch in "a short treatise on fungi".chars() {
        if let Vacant(dupe_entry) = dupes.entry(ch) {
            match singles.entry(ch) {
                Vacant(single_entry) => {
                    single_entry.insert();
                }
                Occupied(single_entry) => {
                    single_entry.remove();
                    dupe_entry.insert();
                }
            }
        }
    }

    assert!(!singles.contains(&'t') && dupes.contains(&'t'));
    assert!(singles.contains(&'u') && !dupes.contains(&'u'));
    assert!(!singles.contains(&'v') && !dupes.contains(&'v'));
}

