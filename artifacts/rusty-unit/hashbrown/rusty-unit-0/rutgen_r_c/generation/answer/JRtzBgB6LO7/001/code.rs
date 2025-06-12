// Answer 0

#[test]
fn test_extract_if_even_elements() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(8, DefaultHashBuilder, Global);
    for x in 0..8 {
        map.insert(x, x);
    }

    let mut drained = map.extract_if(|k, _v| k % 2 == 0).collect::<Vec<_>>();

    let mut evens = drained.iter().map(|(k, _)| *k).collect::<Vec<_>>();
    evens.sort();

    assert_eq!(evens, vec![0, 2, 4, 6]);
    assert_eq!(map.len(), 4);
}

#[test]
fn test_extract_if_odd_elements() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(8, DefaultHashBuilder, Global);
    for x in 0..8 {
        map.insert(x, x);
    }

    let drained = map.extract_if(|k, _v| k % 2 != 0).collect::<Vec<_>>();

    let mut odds = drained.iter().map(|(k, _)| *k).collect::<Vec<_>>();
    odds.sort();

    assert_eq!(odds, vec![1, 3, 5, 7]);
    assert_eq!(map.len(), 4);
}

#[test]
fn test_extract_if_no_elements() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(8, DefaultHashBuilder, Global);

    for x in 0..8 {
        map.insert(x, x);
    }

    {
        let _drained = map.extract_if(|k, _v| k > &8); 
    }

    assert_eq!(map.len(), 8);
}

#[test]
fn test_extract_if_half_elements() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(8, DefaultHashBuilder, Global);
    for x in 0..8 {
        map.insert(x, x);
    }

    let drained = map.extract_if(|k, _v| k < &4).collect::<Vec<_>>();

    let mut less_than_four = drained.iter().map(|(k, _)| *k).collect::<Vec<_>>();
    less_than_four.sort();

    assert_eq!(less_than_four, vec![0, 1, 2, 3]);
    assert_eq!(map.len(), 4);
}

