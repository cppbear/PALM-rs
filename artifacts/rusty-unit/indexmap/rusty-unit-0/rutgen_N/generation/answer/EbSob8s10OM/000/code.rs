// Answer 0

#[derive(Debug)]
struct Bucket<T> {
    value: T,
}

impl<T> Bucket<T> {
    fn key_ref(&self) -> &T {
        &self.value
    }
}

struct Set<T> {
    entries: Vec<Bucket<T>>,
}

impl<T> Set<T> {
    pub fn as_entries(&self) -> &Vec<Bucket<T>> {
        &self.entries
    }

    pub fn first(&self) -> Option<&T> {
        self.as_entries().first().map(Bucket::key_ref)
    }
}

#[test]
fn test_first_empty_set() {
    let set: Set<i32> = Set { entries: Vec::new() };
    assert_eq!(set.first(), None);
}

#[test]
fn test_first_single_element_set() {
    let set = Set {
        entries: vec![Bucket { value: 10 }],
    };
    assert_eq!(set.first(), Some(&10));
}

#[test]
fn test_first_multiple_elements_set() {
    let set = Set {
        entries: vec![Bucket { value: 20 }, Bucket { value: 30 }],
    };
    assert_eq!(set.first(), Some(&20));
}

