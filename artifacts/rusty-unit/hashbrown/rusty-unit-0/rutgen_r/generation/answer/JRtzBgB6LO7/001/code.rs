// Answer 0

#[test]
fn test_extract_if_even_keys() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x)).collect();
    let drained: Vec<(i32, i32)> = map.extract_if(|k, _v| k % 2 == 0).collect();

    let mut evens = drained.iter().map(|(k, _)| *k).collect::<Vec<_>>();
    let mut odds = map.keys().cloned().collect::<Vec<_>>();
    evens.sort();
    odds.sort();

    assert_eq!(evens, vec![0, 2, 4, 6]);
    assert_eq!(odds, vec![1, 3, 5, 7]);
}

#[test]
fn test_extract_if_odd_keys() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x)).collect();
    let drained: Vec<(i32, i32)> = map.extract_if(|k, _v| k % 2 != 0).collect();

    let mut odds = drained.iter().map(|(k, _)| *k).collect::<Vec<_>>();
    let mut evens = map.keys().cloned().collect::<Vec<_>>();
    odds.sort();
    evens.sort();

    assert_eq!(odds, vec![1, 3, 5, 7]);
    assert_eq!(evens, vec![0, 2, 4, 6]);
}

#[test]
fn test_extract_if_no_elements_drained() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x)).collect();
    {
        let _drained = map.extract_if(|k, _v| k % 2 != 0);
    }
    assert_eq!(map.len(), 8); // ExtractIf was not exhausted
}

