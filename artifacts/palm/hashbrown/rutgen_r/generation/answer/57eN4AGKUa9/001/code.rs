// Answer 0

#[test]
fn test_retain_even_elements() {
    use hashbrown::HashSet;

    let xs = [1, 2, 3, 4, 5, 6];
    let mut set: HashSet<i32> = xs.into_iter().collect();
    set.retain(|&k| k % 2 == 0);
    assert_eq!(set.len(), 3);
    assert!(set.contains(&2));
    assert!(set.contains(&4));
    assert!(set.contains(&6));
}

#[test]
fn test_retain_no_elements() {
    use hashbrown::HashSet;

    let xs: [i32; 0] = [];
    let mut set: HashSet<i32> = xs.into_iter().collect();
    set.retain(|&k| k % 2 == 0);
    assert_eq!(set.len(), 0);
}

#[test]
fn test_retain_all_elements() {
    use hashbrown::HashSet;

    let xs = [2, 4, 6, 8, 10];
    let mut set: HashSet<i32> = xs.into_iter().collect();
    set.retain(|&k| k % 2 == 0);
    assert_eq!(set.len(), 5);
}

#[test]
fn test_retain_no_even_elements() {
    use hashbrown::HashSet;

    let xs = [1, 3, 5, 7, 9];
    let mut set: HashSet<i32> = xs.into_iter().collect();
    set.retain(|&k| k % 2 == 0);
    assert_eq!(set.len(), 0);
}

#[test]
fn test_retain_single_element_success() {
    use hashbrown::HashSet;

    let xs = [1];
    let mut set: HashSet<i32> = xs.into_iter().collect();
    set.retain(|&k| k % 2 == 0);
    assert_eq!(set.len(), 0);
}

#[test]
fn test_retain_single_element_failure() {
    use hashbrown::HashSet;

    let xs = [2];
    let mut set: HashSet<i32> = xs.into_iter().collect();
    set.retain(|&k| k % 2 == 0);
    assert_eq!(set.len(), 1);
    assert!(set.contains(&2));
}

