// Answer 0

#[test]
fn test_union_basic() {
    let mut a: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() }};
    let mut b: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() }};

    a.insert(1);
    a.insert(2);
    a.insert(3);
    b.insert(2);
    b.insert(3);
    b.insert(4);

    let union_set = a.union(&b);
    let union: HashSet<i32> = union_set.collect();

    assert_eq!(union.len(), 4);
    assert!(union.contains(&1));
    assert!(union.contains(&2));
    assert!(union.contains(&3));
    assert!(union.contains(&4));
}

#[test]
fn test_union_empty_sets() {
    let a: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() }};
    let b: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() }};

    let union_set = a.union(&b);
    let union: HashSet<i32> = union_set.collect();

    assert!(union.is_empty());
}

#[test]
fn test_union_with_one_empty() {
    let mut a: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() }};
    let b: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() }};

    a.insert(1);
    a.insert(2);

    let union_set = a.union(&b);
    let union: HashSet<i32> = union_set.collect();

    assert_eq!(union.len(), 2);
    assert!(union.contains(&1));
    assert!(union.contains(&2));
}

#[test]
fn test_union_duplicate_elements() {
    let mut a: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() }};
    let mut b: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() }};

    a.insert(1);
    a.insert(2);
    b.insert(2);
    b.insert(3);
    b.insert(4); 

    let union_set = a.union(&b);
    let union: HashSet<i32> = union_set.collect();

    assert_eq!(union.len(), 4);
    assert!(union.contains(&1));
    assert!(union.contains(&2));
    assert!(union.contains(&3));
    assert!(union.contains(&4));
}

