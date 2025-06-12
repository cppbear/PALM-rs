// Answer 0

#[test]
fn test_extract_if_empty_set() {
    let mut set: HashSet<i32> = HashSet::default(); // assumes default initializes an empty set
    let extracted: ExtractIf<i32, _> = set.extract_if(|_| true);
    assert!(set.len() == 0);
    assert!(extracted.inner.iter.items == 0);
}

#[test]
fn test_extract_if_no_elements_meet_condition() {
    let mut set: HashSet<i32> = (1..10).collect();
    let extracted: ExtractIf<i32, _> = set.extract_if(|&v| v > 10);
    
    let drained: Vec<_> = extracted.inner.iter.collect();
    assert!(drained.is_empty());
    assert!(set.len() == 9); // no elements removed
}

#[test]
fn test_extract_if_all_elements_meet_condition() {
    let mut set: HashSet<i32> = (1..5).collect();
    let extracted: ExtractIf<i32, _> = set.extract_if(|_| true);
    
    let drained: Vec<_> = extracted.inner.iter.collect();
    assert_eq!(drained, vec![1, 2, 3, 4]);
    assert!(set.len() == 0); // all elements removed
}

#[test]
fn test_extract_if_some_elements_meet_condition() {
    let mut set: HashSet<i32> = (0..10).collect();
    let extracted: ExtractIf<i32, _> = set.extract_if(|&v| v % 2 == 0);

    let drained: Vec<_> = extracted.inner.iter.collect();
    assert_eq!(drained, vec![0, 2, 4, 6, 8]); // all even numbers expected
    assert_eq!(set.len(), 5); // should have only odd numbers remaining
}

#[test]
#[should_panic]
fn test_extract_if_panic_on_empty_extraction() {
    let mut set: HashSet<i32> = HashSet::default(); // an empty set
    let extracted: ExtractIf<i32, _> = set.extract_if(|_| panic!("This should panic!"));
    extracted.inner.iter.collect(); // invoking the extraction should panic
}

