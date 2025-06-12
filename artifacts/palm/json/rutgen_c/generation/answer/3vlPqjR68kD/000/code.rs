// Answer 0

#[test]
fn test_size_hint_equal_lower_upper() {
    struct TestMapIter {
        data: Vec<(String, Value)>,
        index: usize,
    }

    impl TestMapIter {
        fn new(data: Vec<(String, Value)>) -> Self {
            Self { data, index: 0 }
        }
    }

    impl Iterator for TestMapIter {
        type Item = (String, Value);

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index].clone();
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = self.data.len() - self.index;
            (len, Some(len))
        }
    }

    let data = vec![
        (String::from("key1"), Value::Null),
        (String::from("key2"), Value::Bool(true)),
    ];
    
    let iter = TestMapIter::new(data);
    let deserializer = MapRefDeserializer { 
        iter: iter, 
        value: None 
    };
    
    assert_eq!(deserializer.size_hint(), Some(2));
}

#[test]
fn test_size_hint_different_lower_upper() {
    struct TestMapIter {
        data: Vec<(String, Value)>,
        index: usize,
    }

    impl TestMapIter {
        fn new(data: Vec<(String, Value)>) -> Self {
            Self { data, index: 0 }
        }
    }

    impl Iterator for TestMapIter {
        type Item = (String, Value);

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.data.len() {
                let item = self.data[self.index].clone();
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = self.data.len() - self.index + 1; // Intentional difference
            (len, Some(len))
        }
    }

    let data = vec![
        (String::from("key1"), Value::Null),
        (String::from("key2"), Value::Bool(true)),
    ];
    
    let iter = TestMapIter::new(data);
    let deserializer = MapRefDeserializer { 
        iter: iter, 
        value: None 
    };
    
    assert_eq!(deserializer.size_hint(), None);
}

