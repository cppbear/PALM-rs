// Answer 0

#[test]
fn test_iter_mut_empty_map() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder::new(), Global);
    let mut iter = map.iter_mut();
    assert!(iter.is_empty());
}

#[test]
fn test_iter_mut_single_element() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    {
        let mut iter = map.iter_mut();
        assert_eq!(iter.next(), Some(("a", &mut 1)));
    }
}

#[test]
fn test_iter_mut_multiple_elements() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    {
        let mut iter = map.iter_mut();
        for (_, val) in &mut iter {
            *val *= 2;
        }
    }

    let results: Vec<(&str, i32)> = map.iter().map(|(k, v)| (*k, *v)).collect();
    assert_eq!(results, &[("a", 2), ("b", 4), ("c", 6)]);
}

