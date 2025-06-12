// Answer 0

#[derive(Debug)]
struct ClassUnicode {
    set: Vec<u32>,
}

impl ClassUnicode {
    pub fn iter(&self) -> ClassUnicodeIter {
        ClassUnicodeIter(self.set.iter())
    }
}

struct ClassUnicodeIter<'a>(std::slice::Iter<'a, u32>);

impl<'a> Iterator for ClassUnicodeIter<'a> {
    type Item = &'a u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[test]
fn test_iter_empty() {
    let unicode_class = ClassUnicode { set: vec![] };
    let mut iter = unicode_class.iter();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_single_element() {
    let unicode_class = ClassUnicode { set: vec![10] };
    let mut iter = unicode_class.iter();
    assert_eq!(iter.next(), Some(&10));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_multiple_elements() {
    let unicode_class = ClassUnicode { set: vec![5, 15, 25] };
    let mut iter = unicode_class.iter();
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&15));
    assert_eq!(iter.next(), Some(&25));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_order() {
    let unicode_class = ClassUnicode { set: vec![3, 1, 2] };
    let mut iter = unicode_class.iter();
    let results: Vec<_> = iter.collect();
    assert_eq!(results, vec![&3, &1, &2]);
}

