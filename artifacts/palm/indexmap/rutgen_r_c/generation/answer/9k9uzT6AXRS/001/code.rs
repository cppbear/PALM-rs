// Answer 0

#[test]
fn test_split_splice_valid_range() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: 0, value: 10 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 20 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 30 });
    
    let (split, iter) = map.split_splice(1..2);
    
    assert_eq!(split.entries.len(), 2);
    assert_eq!(split.entries[0].key, 1);
    assert_eq!(split.entries[1].key, 2);
    
    let drained: Vec<_> = iter.collect();
    assert_eq!(drained.len(), 1);
    assert_eq!(drained[0].key, 1);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_split_splice_panic_out_of_bounds() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: 0, value: 10 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 20 });
    
    let _ = map.split_splice(0..3);
}

#[test]
fn test_split_splice_empty_map() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    
    let (split, iter) = map.split_splice(0..0);
    
    assert_eq!(split.entries.len(), 0);
    let drained: Vec<_> = iter.collect();
    assert_eq!(drained.len(), 0);
}

#[test]
fn test_split_splice_entire_range() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: 0, value: 10 });
    map.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 20 });

    let (split, iter) = map.split_splice(0..2);
    
    assert_eq!(split.entries.len(), 0);
    let drained: Vec<_> = iter.collect();
    assert_eq!(drained.len(), 2);
    assert_eq!(drained[0].key, 0);
    assert_eq!(drained[1].key, 1);
}

#[test]
fn test_split_splice_single_element() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: 0, value: 10 });
    
    let (split, iter) = map.split_splice(0..1);
    
    assert_eq!(split.entries.len(), 0);
    let drained: Vec<_> = iter.collect();
    assert_eq!(drained.len(), 1);
    assert_eq!(drained[0].key, 0);
}

