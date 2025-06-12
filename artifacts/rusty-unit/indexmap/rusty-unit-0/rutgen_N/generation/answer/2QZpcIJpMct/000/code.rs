// Answer 0

#[test]
fn test_new_with_exclusive_range() {
    use indexmap::IndexSet;
    use std::ops::Range;

    struct UnitValue<I>(I);

    let mut set = IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    let range = 1..2;
    let replace_with = 4;

    let result = new(&mut set, range, replace_with);
    
    let expected_set: Vec<_> = set.iter().cloned().collect();
    assert_eq!(expected_set, vec![1, 4, 3]);
}

#[test]
fn test_new_with_empty_range() {
    use indexmap::IndexSet;
    use std::ops::Range;

    struct UnitValue<I>(I);

    let mut set = IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    let range = 2..2;
    let replace_with = 5;

    let result = new(&mut set, range, replace_with);
    
    let expected_set: Vec<_> = set.iter().cloned().collect();
    assert_eq!(expected_set, vec![1, 2, 3]);
}

#[test]
fn test_new_with_full_range() {
    use indexmap::IndexSet;
    use std::ops::Range;

    struct UnitValue<I>(I);

    let mut set = IndexSet::new();
    set.insert(10);
    set.insert(20);
    set.insert(30);

    let range = 0..3;
    let replace_with = 100;

    let result = new(&mut set, range, replace_with);
    
    let expected_set: Vec<_> = set.iter().cloned().collect();
    assert_eq!(expected_set, vec![100]);
}

