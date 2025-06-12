// Answer 0

#[test]
fn test_drain_valid_range() {
    #[derive(Debug)]
    struct Keys(usize);
    #[derive(Debug)]
    struct Values(usize);
    
    let mut map: IndexMapCore<Keys, Values> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: Keys(0), value: Values(0) });
    map.entries.push(Bucket { hash: HashValue::default(), key: Keys(1), value: Values(1) });
    map.entries.push(Bucket { hash: HashValue::default(), key: Keys(2), value: Values(2) });

    let drained: Vec<_> = map.drain(1..3).collect();
    assert_eq!(drained.len(), 2);
    assert_eq!(drained[0].key.0, 1);
    assert_eq!(drained[1].key.0, 2);
    assert_eq!(map.entries.len(), 1);
    assert_eq!(map.entries[0].key.0, 0);
}

#[test]
#[should_panic(expected = "range start index")]
fn test_drain_out_of_bounds_start() {
    #[derive(Debug)]
    struct Keys(usize);
    #[derive(Debug)]
    struct Values(usize);

    let mut map: IndexMapCore<Keys, Values> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: Keys(0), value: Values(0) });

    let _drained: Vec<_> = map.drain(2..3).collect();
}

#[test]
#[should_panic(expected = "range end index")]
fn test_drain_out_of_bounds_end() {
    #[derive(Debug)]
    struct Keys(usize);
    #[derive(Debug)]
    struct Values(usize);

    let mut map: IndexMapCore<Keys, Values> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue::default(), key: Keys(0), value: Values(0) });

    let _drained: Vec<_> = map.drain(0..2).collect();
}

#[test]
fn test_drain_empty_map() {
    #[derive(Debug)]
    struct Keys(usize);
    #[derive(Debug)]
    struct Values(usize);

    let mut map: IndexMapCore<Keys, Values> = IndexMapCore::new();

    let drained: Vec<_> = map.drain(0..1).collect();
    assert!(drained.is_empty());
    assert_eq!(map.entries.len(), 0);
}

