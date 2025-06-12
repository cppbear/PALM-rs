// Answer 0

#[test]
fn test_iter_on_empty_map() {
    use std::marker::PhantomData;
    use hashbrown::HashMap;

    struct Iter<'a, K, V> {
        inner: std::slice::Iter<'a, (K, V)>,
        marker: PhantomData<&'a (K, V)>,
    }

    let map: HashMap<i32, i32> = HashMap::new();
    let iter = map.iter();

    assert!(iter.inner.len() == 0);
}

#[test]
fn test_iter_on_non_empty_map() {
    use std::marker::PhantomData;
    use hashbrown::HashMap;

    struct Iter<'a, K, V> {
        inner: std::slice::Iter<'a, (K, V)>,
        marker: PhantomData<&'a (K, V)>,
    }

    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let iter = map.iter();

    let items: Vec<_> = iter.inner.collect();
    assert_eq!(items.len(), 3);
    assert!(items.contains(&(1, 10)));
    assert!(items.contains(&(2, 20)));
    assert!(items.contains(&(3, 30)));
}

#[test]
fn test_iter_on_removal() {
    use std::marker::PhantomData;
    use hashbrown::HashMap;

    struct Iter<'a, K, V> {
        inner: std::slice::Iter<'a, (K, V)>,
        marker: PhantomData<&'a (K, V)>,
    }

    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let iter_before_removal = map.iter();

    assert_eq!(iter_before_removal.inner.len(), 2);

    map.remove(&1);
    let iter_after_removal = map.iter();

    let items: Vec<_> = iter_after_removal.inner.collect();
    assert_eq!(items.len(), 1);
    assert!(!items.contains(&(1, 10)));
    assert!(items.contains(&(2, 20)));
}

