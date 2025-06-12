// Answer 0

#[test]
fn test_replace_existing_value() {
    use hashbrown::HashSet;

    let mut set = HashSet::new();
    let old_value = Vec::<i32>::new();
    set.insert(old_value.clone());

    let replaced_value = set.replace(vec![1, 2, 3]);
    assert_eq!(replaced_value, Some(old_value));
    assert_eq!(set.get(&[][..]).unwrap().capacity(), 3);
}

#[test]
fn test_replace_non_existing_value() {
    use hashbrown::HashSet;

    let mut set = HashSet::new();
    let value = Vec::<i32>::with_capacity(10);

    let replaced_value = set.replace(value.clone());
    assert_eq!(replaced_value, None);
    assert_eq!(set.get(&[][..]).unwrap().capacity(), 10);
}

#[test]
fn test_replace_with_empty_vector() {
    use hashbrown::HashSet;

    let mut set = HashSet::new();
    let old_value = Vec::<i32>::with_capacity(5);
    set.insert(old_value.clone());

    let replaced_value = set.replace(Vec::<i32>::new());
    assert_eq!(replaced_value, Some(old_value));
    assert_eq!(set.get(&[][..]).unwrap().capacity(), 0);
}

