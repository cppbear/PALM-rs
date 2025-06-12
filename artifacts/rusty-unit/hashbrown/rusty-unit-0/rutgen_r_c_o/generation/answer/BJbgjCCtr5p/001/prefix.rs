// Answer 0

#[test]
fn test_symmetric_difference_non_empty_sets() {
    let mut a: HashSet<i32> = HashSet::default();
    let mut b: HashSet<i32> = HashSet::default();
    a.insert(1);
    a.insert(2);
    a.insert(3);
    b.insert(4);
    b.insert(2);
    b.insert(3);
    let _ = a.symmetric_difference(&b);
}

#[test]
fn test_symmetric_difference_empty_first_set() {
    let mut a: HashSet<i32> = HashSet::default();
    let mut b: HashSet<i32> = HashSet::default();
    b.insert(1);
    b.insert(2);
    let _ = a.symmetric_difference(&b);
}

#[test]
fn test_symmetric_difference_empty_second_set() {
    let mut a: HashSet<i32> = HashSet::default();
    let mut b: HashSet<i32> = HashSet::default();
    a.insert(1);
    a.insert(2);
    let _ = a.symmetric_difference(&b);
}

#[test]
fn test_symmetric_difference_both_empty_sets() {
    let a: HashSet<i32> = HashSet::default();
    let b: HashSet<i32> = HashSet::default();
    let _ = a.symmetric_difference(&b);
}

#[test]
fn test_symmetric_difference_disjoint_sets() {
    let mut a: HashSet<i32> = HashSet::default();
    let mut b: HashSet<i32> = HashSet::default();
    a.insert(1);
    a.insert(2);
    b.insert(3);
    b.insert(4);
    let _ = a.symmetric_difference(&b);
}

#[test]
fn test_symmetric_difference_identical_sets() {
    let mut a: HashSet<i32> = HashSet::default();
    a.insert(1);
    a.insert(2);
    a.insert(3);
    let _ = a.symmetric_difference(&a);
}

#[test]
fn test_symmetric_difference_large_sets() {
    let mut a: HashSet<i32> = HashSet::default();
    let mut b: HashSet<i32> = HashSet::default();
    for i in 0..10000 {
        a.insert(i);
    }
    for i in 10000..20000 {
        b.insert(i);
    }
    let _ = a.symmetric_difference(&b);
}

