// Answer 0

#[test]
fn test_extract_if_collects_correct_elements() {
    use crate::HashSet;
    
    let mut set: HashSet<i32> = (0..8).collect();
    let drained: ExtractIf<_, _> = set.extract_if(|v| v % 2 == 0);

    let mut evens: Vec<i32> = drained.collect();
    let mut odds: Vec<i32> = set.into_iter().collect();
    
    evens.sort();
    odds.sort();

    assert_eq!(evens, vec![0, 2, 4, 6]);
    assert_eq!(odds, vec![1, 3, 5, 7]);
}

#[test]
fn test_extract_if_partial_extraction() {
    use crate::HashSet;

    let mut set: HashSet<i32> = (0..8).collect();
    let mut drained: ExtractIf<_, _> = set.extract_if(|v| {
        if *v == 4 {
            false // short-circuiting at 4
        } else {
            true
        }
    });

    assert_eq!(drained.next(), Some(0));
    assert_eq!(drained.next(), Some(2));
    assert_eq!(drained.next(), Some(6));
    
    let remaining: Vec<i32> = set.into_iter().collect();
    assert_eq!(remaining, vec![1, 3, 4, 5, 7]);
}

#[test]
fn test_extract_if_empty_set() {
    use crate::HashSet;

    let mut set: HashSet<i32> = HashSet::new();
    let drained: ExtractIf<_, _> = set.extract_if(|_| true);
    
    let collected: Vec<i32> = drained.collect();
    assert!(collected.is_empty());
    assert!(set.is_empty());
} 

#[test]
fn test_extract_if_no_elements_meet_predicate() {
    use crate::HashSet;

    let mut set: HashSet<i32> = (0..8).collect();
    let drained: ExtractIf<_, _> = set.extract_if(|_| false);
    
    let collected: Vec<i32> = drained.collect();
    assert!(collected.is_empty());
    assert_eq!(set.len(), 8);
} 

