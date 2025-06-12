// Answer 0

#[derive(Debug)]
struct TestMap {
    data: Vec<(i32, i32)>,
}

impl TestMap {
    fn new(data: Vec<(i32, i32)>) -> Self {
        TestMap { data }
    }

    fn as_slice(&self) -> &[(i32, i32)] {
        &self.data
    }

    fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&i32, &i32) -> bool,
    {
        self.as_slice().iter().position(|&(k, v)| !pred(&k, &v)).unwrap_or(self.data.len())
    }
}

#[test]
fn test_partition_point_empty() {
    let map = TestMap::new(vec![]);
    let pred = |&k, &v| k < v;
    assert_eq!(map.partition_point(pred), 0);
}

#[test]
fn test_partition_point_all_satisfy() {
    let map = TestMap::new(vec![(1, 2), (2, 3), (3, 4)]);
    let pred = |&k, &v| k < v;
    assert_eq!(map.partition_point(pred), 3);
}

#[test]
fn test_partition_point_none_satisfy() {
    let map = TestMap::new(vec![(2, 1), (3, 2), (4, 3)]);
    let pred = |&k, &v| k < v;
    assert_eq!(map.partition_point(pred), 0);
}

#[test]
fn test_partition_point_mixed() {
    let map = TestMap::new(vec![(1, 1), (2, 3), (3, 2), (4, 4)]);
    let pred = |&k, &v| k <= v;
    assert_eq!(map.partition_point(pred), 2);
}

#[test]
fn test_partition_point_boundaries() {
    let map = TestMap::new(vec![(1, 1), (2, 2), (3, 3)]);
    let pred = |&k, &v| k == v;
    assert_eq!(map.partition_point(pred), 3);
}

