// Answer 0

#[test]
fn test_bitor_assign_empty_sets() {
    let mut a: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() } };
    let b: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() } };

    a |= &b;

    assert!(a.is_empty());
}

#[test]
fn test_bitor_assign_no_common_elements() {
    let mut a: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() } };
    a.insert(1);
    a.insert(2);
    
    let b: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() } };
    b.insert(3);
    b.insert(4);
    
    a |= &b;

    let expected: Vec<i32> = vec![1, 2, 3, 4];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_bitor_assign_with_common_elements() {
    let mut a: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() } };
    a.insert(1);
    a.insert(2);
    
    let b: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() } };
    b.insert(2);
    b.insert(3);
    
    a |= &b;

    let expected: Vec<i32> = vec![1, 2, 3];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

#[test]
fn test_bitor_assign_duplicates_in_rhs() {
    let mut a: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() } };
    a.insert(1);
  
    let b: HashSet<i32> = HashSet { map: HashMap { hash_builder: DefaultHashBuilder {}, table: RawTable::new() } };
    b.insert(1);
    b.insert(2);
  
    a |= &b;

    let expected: Vec<i32> = vec![1, 2];
    let mut i = 0;
    for x in &a {
        assert!(expected.contains(x));
        i += 1;
    }
    assert_eq!(i, expected.len());
}

