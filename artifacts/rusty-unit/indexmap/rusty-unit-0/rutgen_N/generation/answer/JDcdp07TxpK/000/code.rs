// Answer 0

#[derive(Debug)]
struct Entry<T> {
    key: T,
}

struct IntoIter<T> {
    entries: Vec<Entry<T>>,
    index: usize,
}

impl<T> IntoIter<T> {
    fn new(entries: Vec<Entry<T>>) -> Self {
        Self { entries, index: 0 }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = Entry<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.entries.len() {
            let entry = self.entries[self.index].clone();
            self.index += 1;
            Some(entry)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Set<T> {
    entries: Vec<Entry<T>>,
}

impl<T: Clone> Set<T> {
    fn into_entries(self) -> Vec<Entry<T>> {
        self.entries
    }

    pub fn sorted_by<F>(self, mut cmp: F) -> IntoIter<T>
    where
        F: FnMut(&T, &T) -> std::cmp::Ordering,
    {
        let mut entries = self.into_entries();
        entries.sort_by(move |a, b| cmp(&a.key, &b.key));
        IntoIter::new(entries)
    }
}

#[test]
fn test_sorted_by_empty_set() {
    let set: Set<i32> = Set { entries: vec![] };
    let mut iter = set.sorted_by(|a, b| a.cmp(b));
    assert!(iter.next().is_none());
}

#[test]
fn test_sorted_by_single_element() {
    let set = Set {
        entries: vec![Entry { key: 1 }],
    };
    let mut iter = set.sorted_by(|a, b| a.cmp(b));
    assert_eq!(iter.next().unwrap().key, 1);
    assert!(iter.next().is_none());
}

#[test]
fn test_sorted_by_multiple_elements() {
    let set = Set {
        entries: vec![
            Entry { key: 3 },
            Entry { key: 1 },
            Entry { key: 2 },
        ],
    };
    let mut iter = set.sorted_by(|a, b| a.cmp(b));
    assert_eq!(iter.next().unwrap().key, 1);
    assert_eq!(iter.next().unwrap().key, 2);
    assert_eq!(iter.next().unwrap().key, 3);
    assert!(iter.next().is_none());
}

#[test]
fn test_sorted_by_reverse_order() {
    let set = Set {
        entries: vec![
            Entry { key: 2 },
            Entry { key: 3 },
            Entry { key: 1 },
        ],
    };
    let mut iter = set.sorted_by(|a, b| b.cmp(a));
    assert_eq!(iter.next().unwrap().key, 3);
    assert_eq!(iter.next().unwrap().key, 2);
    assert_eq!(iter.next().unwrap().key, 1);
    assert!(iter.next().is_none());
}

