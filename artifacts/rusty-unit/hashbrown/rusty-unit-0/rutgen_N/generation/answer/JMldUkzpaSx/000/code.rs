// Answer 0

#[test]
fn test_entry_increments_existing_key() {
    use hashbrown::HashMap;

    let mut letters = HashMap::new();
    for ch in "hello world".chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }

    assert_eq!(letters['h'], 1);
    assert_eq!(letters['e'], 1);
    assert_eq!(letters['l'], 3);
    assert_eq!(letters['o'], 2);
    assert_eq!(letters['w'], 1);
    assert_eq!(letters['r'], 1);
    assert_eq!(letters['d'], 1);
}

#[test]
fn test_entry_inserts_new_key() {
    use hashbrown::HashMap;

    let mut letters = HashMap::new();
    let counter = letters.entry('x').or_insert(0);
    *counter += 1;

    assert_eq!(letters['x'], 1);
}

#[test]
fn test_entry_for_non_existent_key() {
    use hashbrown::HashMap;

    let mut letters = HashMap::new();
    assert_eq!(letters.get(&'y'), None);
    let counter = letters.entry('y').or_insert(0);
    *counter += 1;

    assert_eq!(letters['y'], 1);
}

#[test]
fn test_entry_for_key_with_zero_initialization() {
    use hashbrown::HashMap;

    let mut letters = HashMap::new();
    let counter = letters.entry('z').or_insert(0);

    assert_eq!(*counter, 0);
    *counter += 1;

    assert_eq!(letters['z'], 1);
}

