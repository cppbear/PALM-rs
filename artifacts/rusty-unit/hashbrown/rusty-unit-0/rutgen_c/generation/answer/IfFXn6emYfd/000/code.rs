// Answer 0

#[test]
fn test_get_or_insert_with_insert_new_value() {
    use hashbrown::HashSet;

    let mut set: HashSet<String> = ["cat", "dog", "horse"]
        .iter()
        .map(|&pet| pet.to_owned())
        .collect();

    assert_eq!(set.len(), 3);
    for &pet in &["cat", "dog", "fish"] {
        let value = set.get_or_insert_with(pet, str::to_owned);
        assert_eq!(value, pet);
    }
    assert_eq!(set.len(), 4); // a new "fish" was inserted
}

#[test]
#[should_panic]
fn test_get_or_insert_with_panic_on_non_equivalent() {
    use hashbrown::HashSet;

    let mut set = HashSet::new();
    set.get_or_insert_with("rust", |_| String::new());
}

