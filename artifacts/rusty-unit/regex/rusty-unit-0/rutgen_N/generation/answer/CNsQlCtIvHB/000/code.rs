// Answer 0

#[derive(Debug)]
struct ClassBytes {
    set: Vec<u8>,
}

impl ClassBytes {
    pub fn new(set: Vec<u8>) -> Self {
        ClassBytes { set }
    }

    pub fn iter(&self) -> ClassBytesIter {
        ClassBytesIter(self.set.iter())
    }
}

struct ClassBytesIter<'a>(std::slice::Iter<'a, u8>);

impl<'a> Iterator for ClassBytesIter<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[test]
fn test_iter_empty() {
    let class_bytes = ClassBytes::new(vec![]);
    let mut iter = class_bytes.iter();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_single_element() {
    let class_bytes = ClassBytes::new(vec![42]);
    let mut iter = class_bytes.iter();
    assert_eq!(iter.next(), Some(&42));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_multiple_elements() {
    let class_bytes = ClassBytes::new(vec![1, 2, 3]);
    let mut iter = class_bytes.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

