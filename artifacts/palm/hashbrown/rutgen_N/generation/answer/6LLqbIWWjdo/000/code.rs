// Answer 0

#[test]
fn test_extract_if_removes_even_numbers() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = (0..8).collect();
    let drained: Vec<i32> = set.extract_if(|v| v % 2 == 0).collect();

    let mut evens = drained.clone();
    let mut odds = set.into_iter().collect::<Vec<_>>();
    evens.sort();
    odds.sort();

    assert_eq!(evens, vec![0, 2, 4, 6]);
    assert_eq!(odds, vec![1, 3, 5, 7]);
}

#[test]
fn test_extract_if_no_elements() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::new();
    let drained: Vec<i32> = set.extract_if(|v| v % 2 == 0).collect();

    assert!(drained.is_empty());
    assert!(set.is_empty());
}

#[test]
fn test_extract_if_all_elements() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = (0..10).collect();
    let drained: Vec<i32> = set.extract_if(|v| *v < 10).collect();

    let mut extracted = drained.clone();
    let remaining: Vec<i32> = set.into_iter().collect();
    extracted.sort();
    remaining.sort();

    assert_eq!(extracted, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert!(remaining.is_empty());
}

#[test]
fn test_extract_if_some_elements() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = (0..5).collect();
    let drained: Vec<i32> = set.extract_if(|v| *v == 2 || *v == 3).collect();

    let mut extracted = drained.clone();
    let remaining: Vec<i32> = set.into_iter().collect();
    extracted.sort();
    remaining.sort();

    assert_eq!(extracted, vec![2, 3]);
    assert_eq!(remaining, vec![0, 1, 4]);
}

