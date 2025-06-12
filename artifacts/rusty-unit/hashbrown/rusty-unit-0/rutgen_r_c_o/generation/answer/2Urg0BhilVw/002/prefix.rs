// Answer 0

#[test]
fn test_intersection_smaller_self() {
    let mut set_a: HashSet<i32> = HashSet::new();
    set_a.insert(1);
    set_a.insert(2);
    let mut set_b: HashSet<i32> = HashSet::new();
    set_b.insert(1);
    set_b.insert(2);
    set_b.insert(3);
    let _intersection = set_a.intersection(&set_b);
}

#[test]
fn test_intersection_equal_length() {
    let mut set_a: HashSet<i32> = HashSet::new();
    set_a.insert(1);
    set_a.insert(2);
    let mut set_b: HashSet<i32> = HashSet::new();
    set_b.insert(1);
    set_b.insert(2);
    set_b.insert(3);
    let _intersection = set_b.intersection(&set_a);
}

#[test]
fn test_intersection_larger_other() {
    let mut set_a: HashSet<i32> = HashSet::new();
    set_a.insert(4);
    set_a.insert(5);
    let mut set_b: HashSet<i32> = HashSet::new();
    set_b.insert(1);
    set_b.insert(2);
    set_b.insert(3);
    set_b.insert(4);
    let _intersection = set_a.intersection(&set_b);
}

#[test]
fn test_intersection_empty_self() {
    let set_a: HashSet<i32> = HashSet::new();
    let mut set_b: HashSet<i32> = HashSet::new();
    set_b.insert(1);
    set_b.insert(2);
    let _intersection = set_a.intersection(&set_b);
}

#[test]
fn test_intersection_empty_other() {
    let mut set_a: HashSet<i32> = HashSet::new();
    set_a.insert(1);
    let set_b: HashSet<i32> = HashSet::new();
    let _intersection = set_a.intersection(&set_b);
}

