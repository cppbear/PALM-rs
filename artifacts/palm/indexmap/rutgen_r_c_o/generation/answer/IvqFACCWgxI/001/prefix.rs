// Answer 0

#[test]
fn test_with_capacity_zero() {
    let map = IndexMap::with_capacity(0);
}

#[test]
fn test_with_capacity_one() {
    let map = IndexMap::with_capacity(1);
}

#[test]
fn test_with_capacity_ten() {
    let map = IndexMap::with_capacity(10);
}

#[test]
fn test_with_capacity_one_hundred() {
    let map = IndexMap::with_capacity(100);
}

#[test]
fn test_with_capacity_one_thousand() {
    let map = IndexMap::with_capacity(1000);
}

#[test]
#[should_panic]
fn test_with_capacity_max() {
    let map = IndexMap::with_capacity(usize::MAX);
}

