// Answer 0

#[derive(Debug)]
struct RawTable<T> {
    table: Vec<Option<T>>,
}

#[derive(Debug)]
struct RawIter<'a, T: 'a> {
    iter: std::slice::Iter<'a, Option<T>>,
}

impl<T> RawTable<T> {
    fn new() -> Self {
        RawTable { table: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.table.push(Some(value));
    }

    fn iter(&self) -> RawIter<T> {
        RawIter {
            iter: self.table.iter(),
        }
    }
}

impl<'a, T> Iterator for RawIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(option) = self.iter.next() {
            if let Some(ref value) = option {
                return Some(value);
            }
        }
        None
    }
}

#[test]
fn test_iter_empty() {
    let table: RawTable<i32> = RawTable::new();
    let iter = unsafe { table.iter() };
    let collected: Vec<_> = iter.collect();
    assert_eq!(collected, Vec::<&i32>::new());
}

#[test]
fn test_iter_single_element() {
    let mut table = RawTable::new();
    table.push(42);
    let iter = unsafe { table.iter() };
    let collected: Vec<_> = iter.collect();
    assert_eq!(collected, vec![&42]);
}

#[test]
fn test_iter_multiple_elements() {
    let mut table = RawTable::new();
    table.push(1);
    table.push(2);
    table.push(3);
    let iter = unsafe { table.iter() };
    let collected: Vec<_> = iter.collect();
    assert_eq!(collected, vec![&1, &2, &3]);
}

#[test]
fn test_iter_with_none() {
    let mut table = RawTable::new();
    table.push(1);
    table.push(2);
    table.push(3);
    table.table.push(None); // Add a None element
    let iter = unsafe { table.iter() };
    let collected: Vec<_> = iter.collect();
    assert_eq!(collected, vec![&1, &2, &3]);
}

