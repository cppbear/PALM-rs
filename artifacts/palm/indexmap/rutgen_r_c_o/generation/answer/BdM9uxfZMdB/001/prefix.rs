// Answer 0

#[test]
fn test_with_capacity_zero() {
    let _set = IndexSet::<i32>::with_capacity(0);
}

#[test]
fn test_with_capacity_one() {
    let _set = IndexSet::<i32>::with_capacity(1);
}

#[test]
fn test_with_capacity_two() {
    let _set = IndexSet::<i32>::with_capacity(2);
}

#[test]
fn test_with_capacity_three() {
    let _set = IndexSet::<i32>::with_capacity(3);
}

#[test]
fn test_with_capacity_under_1000() {
    let _set = IndexSet::<i32>::with_capacity(999);
}

#[test]
fn test_with_capacity_1000() {
    let _set = IndexSet::<i32>::with_capacity(1000);
}

#[test]
fn test_with_capacity_10000() {
    let _set = IndexSet::<i32>::with_capacity(10000);
}

#[test]
fn test_with_capacity_100000() {
    let _set = IndexSet::<i32>::with_capacity(100000);
}

#[test]
#[should_panic]
fn test_with_capacity_usize_max() {
    let _set = IndexSet::<i32>::with_capacity(usize::MAX);
}

