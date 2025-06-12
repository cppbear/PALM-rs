// Answer 0

#[test]
fn test_get_or_insert_with_existing_value() {
    use hashbrown::HashSet;

    let mut set: HashSet<String> = ["cat", "dog", "horse"]
        .iter().map(|&pet| pet.to_owned()).collect();

    assert_eq!(set.len(), 3);
    for &pet in &["cat", "dog"] { // Existing values
        let value = set.get_or_insert_with(pet, str::to_owned);
        assert_eq!(value, pet);
    }
    assert_eq!(set.len(), 3); // No new values were inserted
}

#[test]
fn test_get_or_insert_with_new_value() {
    use hashbrown::HashSet;

    let mut set: HashSet<String> = ["cat", "dog", "horse"]
        .iter().map(|&pet| pet.to_owned()).collect();

    assert_eq!(set.len(), 3);
    let value = set.get_or_insert_with("fish", str::to_owned); // New value
    assert_eq!(value, "fish");
    assert_eq!(set.len(), 4); // New "fish" was inserted
}

#[should_panic]
fn test_get_or_insert_with_panic_on_non_equivalent() {
    use hashbrown::HashSet;

    let mut set: HashSet<String> = HashSet::new();
    set.get_or_insert_with("rust", |_| String::from("not rust")); // Should panic
}

