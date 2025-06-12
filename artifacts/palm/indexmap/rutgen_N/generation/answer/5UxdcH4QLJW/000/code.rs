// Answer 0

#[derive(Debug)]
struct Bucket<T> {
    value: T,
}

struct MyStruct<T> {
    buckets: Vec<Bucket<T>>,
}

impl<T> MyStruct<T> {
    fn into_boxed(self) -> Box<Self> {
        Box::new(self)
    }
    
    fn into_vec(self: Box<Self>) -> Vec<Bucket<T>> {
        self.buckets
    }
    
    fn new(buckets: Vec<Bucket<T>>) -> Self {
        Self { buckets }
    }
}

pub(crate) fn into_entries(self: Box<MyStruct<T>>) -> Vec<Bucket<T>> {
    self.into_boxed().into_vec()
}

#[test]
fn test_into_entries() {
    let buckets = vec![Bucket { value: 1 }, Bucket { value: 2 }];
    let my_struct = MyStruct::new(buckets);
    let result = into_entries(Box::new(my_struct));

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].value, 1);
    assert_eq!(result[1].value, 2);
}

#[test]
fn test_into_entries_empty() {
    let my_struct = MyStruct::new(vec![]);
    let result = into_entries(Box::new(my_struct));

    assert_eq!(result.len(), 0);
}

