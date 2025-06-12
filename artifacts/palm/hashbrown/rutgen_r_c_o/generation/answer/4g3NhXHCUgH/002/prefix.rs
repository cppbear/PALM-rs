// Answer 0

#[test]
fn test_union_with_empty_other_set() {
    let mut a: HashSet<i32> = HashSet::new();
    a.insert(1);
    a.insert(2);
    let b: HashSet<i32> = HashSet::new();
    let _union = a.union(&b);
}

#[test]
fn test_union_with_smaller_other_set() {
    let mut a: HashSet<i32> = HashSet::new();
    a.insert(1);
    a.insert(2);
    a.insert(3);
    
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(4);
    
    let _union = a.union(&b);
}

#[test]
fn test_union_with_equal_size_sets() {
    let mut a: HashSet<i32> = HashSet::new();
    a.insert(1);
    a.insert(2);
    
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(3);
    b.insert(4);
    
    let _union = a.union(&b);
}

#[test]
fn test_union_with_larger_self_set() {
    let mut a: HashSet<i32> = HashSet::new();
    a.insert(1);
    a.insert(2);
    a.insert(3);
    a.insert(4);
    
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(5);
    b.insert(6);
    
    let _union = a.union(&b);
}

#[test]
fn test_union_with_duplicate_entries() {
    let mut a: HashSet<i32> = HashSet::new();
    a.insert(1);
    a.insert(2);
    
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(2);
    b.insert(3);
    
    let _union = a.union(&b);
}

