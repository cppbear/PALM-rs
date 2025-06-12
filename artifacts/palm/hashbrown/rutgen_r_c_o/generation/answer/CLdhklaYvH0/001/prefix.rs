// Answer 0

#[test]
fn test_is_disjoint_empty_sets() {
    let a: HashSet<_> = HashSet::new();
    let b: HashSet<_> = HashSet::new();
    a.is_disjoint(&b);
}

#[test]
fn test_is_disjoint_no_common_elements() {
    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let mut b: HashSet<_> = HashSet::new();
    b.insert(4);
    a.is_disjoint(&b);
}

#[test]
fn test_is_disjoint_with_common_elements() {
    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let mut b: HashSet<_> = HashSet::new();
    b.insert(1);
    a.is_disjoint(&b);
}

#[test]
fn test_is_disjoint_large_disjoint_sets() {
    let a: HashSet<_> = (0..500_000).collect();
    let b: HashSet<_> = (500_000..1_000_000).collect();
    a.is_disjoint(&b);
}

#[test]
fn test_is_disjoint_large_non_disjoint_sets() {
    let a: HashSet<_> = (0..500_000).collect();
    let mut b: HashSet<_> = HashSet::new();
    b.insert(250_000);
    a.is_disjoint(&b);
}

#[test]
fn test_is_disjoint_sets_with_negative_and_positive() {
    let a: HashSet<_> = [-1, 0, 1].iter().cloned().collect();
    let mut b: HashSet<_> = HashSet::new();
    b.insert(2);
    a.is_disjoint(&b);
}

#[test]
fn test_is_disjoint_with_panic_on_large_common_elements() {
    let a: HashSet<_> = (0..1_000_000).collect();
    let mut b: HashSet<_> = HashSet::new();
    for i in 0..1_000_000 {
        b.insert(i);
    }
    a.is_disjoint(&b);
}

