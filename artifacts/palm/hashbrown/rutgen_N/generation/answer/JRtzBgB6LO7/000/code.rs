// Answer 0

#[test]
fn test_extract_if_with_predicate() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x)).collect();
    
    let drained: HashMap<i32, i32> = map.extract_if(|k, _v| k % 2 == 0).collect();
    
    let mut evens = drained.keys().cloned().collect::<Vec<_>>();
    let mut odds = map.keys().cloned().collect::<Vec<_>>();
    evens.sort();
    odds.sort();
    
    assert_eq!(evens, vec![0, 2, 4, 6]);
    assert_eq!(odds, vec![1, 3, 5, 7]);
}

#[test]
fn test_extract_if_with_iterator_dropped() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x)).collect();

    {   
        let _d = map.extract_if(|k, _v| k % 2 != 0);
    }

    assert_eq!(map.len(), 8);
}

