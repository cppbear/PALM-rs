// Answer 0

#[test]
fn test_extract_if_even_numbers() {
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

    let mut set: HashSet<i32> = (0..5).collect();
    let drained: Vec<i32> = set.extract_if(|v| *v >= 0).collect();

    let mut all_extracted = drained.clone();
    let mut remaining = set.into_iter().collect::<Vec<_>>();
    all_extracted.sort();
    remaining.sort();

    assert_eq!(all_extracted, vec![0, 1, 2, 3, 4]);
    assert!(remaining.is_empty());
}

#[test]
fn test_extract_if_never_extracts() {
    use hashbrown::HashSet;

    let mut set: HashSet<i32> = (0..8).collect();
    let drained: Vec<i32> = set.extract_if(|_v| false).collect();

    let mut remaining = set.into_iter().collect::<Vec<_>>();
    remaining.sort();

    assert!(drained.is_empty());
    assert_eq!(remaining, vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

