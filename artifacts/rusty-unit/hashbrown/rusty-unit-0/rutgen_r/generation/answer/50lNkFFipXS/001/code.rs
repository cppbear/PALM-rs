// Answer 0

#[derive(Clone)]
struct TestMap<K, V> {
    inner: Vec<(K, V)>,
}

struct Iter<'a, K, V> {
    inner: Vec<(K, V)>,
    marker: std::marker::PhantomData<&'a (K, V)>,
}

impl<K: Clone, V: Clone> TestMap<K, V> {
    pub fn new() -> Self {
        TestMap { inner: Vec::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.inner.push((key, value));
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: self.inner.clone(),
            marker: std::marker::PhantomData,
        }
    }
}

#[test]
fn test_iter_empty() {
    let map: TestMap<i32, i32> = TestMap::new();
    let iter = map.iter();
    assert_eq!(iter.inner.len(), 0);
}

#[test]
fn test_iter_single_element() {
    let mut map = TestMap::new();
    map.insert(1, 100);
    let iter = map.iter();
    assert_eq!(iter.inner.len(), 1);
    assert_eq!(iter.inner[0], (1, 100));
}

#[test]
fn test_iter_multiple_elements() {
    let mut map = TestMap::new();
    map.insert(1, 100);
    map.insert(2, 200);
    let iter = map.iter();
    assert_eq!(iter.inner.len(), 2);
    assert_eq!(iter.inner[0], (1, 100));
    assert_eq!(iter.inner[1], (2, 200));
}

#[test]
fn test_iter_clone() {
    let mut map = TestMap::new();
    map.insert(1, 100);
    map.insert(2, 200);
    let iter = map.iter();
    let cloned_iter = iter.clone();
    assert_eq!(cloned_iter.inner.len(), iter.inner.len());
    assert_eq!(cloned_iter.inner, iter.inner);
}

