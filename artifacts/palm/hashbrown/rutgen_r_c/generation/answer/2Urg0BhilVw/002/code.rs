// Answer 0

#[test]
fn test_intersection_large_self_small_other() {
    let mut large_set: HashSet<i32> = HashSet::default();
    let values_large = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for value in values_large.iter() {
        large_set.insert(*value);
    }

    let mut small_set: HashSet<i32> = HashSet::default();
    let values_small = vec![3, 4, 5];
    for value in values_small.iter() {
        small_set.insert(*value);
    }

    let intersection = large_set.intersection(&small_set);
    let intersection_values: Vec<i32> = intersection.iter.collect();
    assert_eq!(intersection_values, vec![3, 4, 5]);
}

#[test]
fn test_intersection_equal_length() {
    let mut set_a: HashSet<i32> = HashSet::default();
    let values_a = vec![1, 2, 3, 4, 5];
    for value in values_a.iter() {
        set_a.insert(*value);
    }

    let mut set_b: HashSet<i32> = HashSet::default();
    let values_b = vec![4, 5, 6, 7, 8];
    for value in values_b.iter() {
        set_b.insert(*value);
    }

    let intersection = set_a.intersection(&set_b);
    let intersection_values: Vec<i32> = intersection.iter.collect();
    assert_eq!(intersection_values, vec![4, 5]);
}

#[test]
fn test_intersection_empty_other() {
    let mut set_a: HashSet<i32> = HashSet::default();
    let values_a = vec![1, 2, 3, 4, 5];
    for value in values_a.iter() {
        set_a.insert(*value);
    }

    let empty_set: HashSet<i32> = HashSet::default();

    let intersection = set_a.intersection(&empty_set);
    let intersection_values: Vec<i32> = intersection.iter.collect();
    assert_eq!(intersection_values, vec![]);
}

