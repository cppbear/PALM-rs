// Answer 0

#[test]
fn test_iter_mut_empty() {
    struct SimpleHashBuilder;
    
    let mut map: IndexMap<i32, i32, SimpleHashBuilder> = IndexMap::with_capacity_and_hasher(0, SimpleHashBuilder);
    let mut iter = map.iter_mut();
    
    assert_eq!(iter.iter.len(), 0);
}

#[test]
fn test_iter_mut_non_empty() {
    struct SimpleHashBuilder;
    
    let mut map: IndexMap<i32, i32, SimpleHashBuilder> = IndexMap::with_capacity_and_hasher(10, SimpleHashBuilder);
    
    // Use a hypothetical method to insert elements into the map, if available.
    map.core.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.core.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });

    let mut iter = map.iter_mut();
    let mut collected: Vec<_> = iter.as_slice().iter().map(|bucket| (&bucket.key, &bucket.value)).collect();

    assert_eq!(collected, vec![(&1, &10), (&2, &20)]);
}

#[test]
fn test_iter_mut_alteration() {
    struct SimpleHashBuilder;
    
    let mut map: IndexMap<i32, i32, SimpleHashBuilder> = IndexMap::with_capacity_and_hasher(10, SimpleHashBuilder);
    
    map.core.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 });
    map.core.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 });

    {
        let mut iter = map.iter_mut();
        if let Some((_, value)) = iter.get_index_mut2(0) {
            *value += 5;
        }
    }

    let mut iter = map.iter_mut();
    let mut collected: Vec<_> = iter.as_slice().iter().map(|bucket| (&bucket.key, &bucket.value)).collect();

    assert_eq!(collected, vec![(&1, &15), (&2, &20)]);
}

