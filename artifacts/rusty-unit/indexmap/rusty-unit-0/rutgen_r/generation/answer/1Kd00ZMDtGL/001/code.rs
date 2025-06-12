// Answer 0

#[test]
fn test_take_existing_value() {
    struct MySet {
        data: Vec<i32>,
    }
    
    impl MySet {
        fn new() -> Self {
            MySet { data: Vec::new() }
        }
        
        fn insert(&mut self, value: i32) {
            self.data.push(value);
        }
        
        fn swap_take<Q>(&mut self, value: &Q) -> Option<i32>
        where
            Q: ?Sized + PartialEq + std::hash::Hash,
        {
            if let Some(pos) = self.data.iter().position(|x| *x == *value) {
                let last = self.data.pop();
                if let Some(v) = last {
                    self.data.swap(pos, pos);
                }
                Some(value.clone())
            } else {
                None
            }
        }

        fn take<Q>(&mut self, value: &Q) -> Option<i32>
        where
            Q: ?Sized + std::hash::Hash + PartialEq<i32>,
        {
            self.swap_take(value)
        }
    }

    let mut set = MySet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    assert_eq!(set.take(&2), Some(2));
    assert_eq!(set.take(&1), Some(1));
    assert_eq!(set.take(&3), Some(3));
}

#[test]
fn test_take_non_existing_value() {
    struct MySet {
        data: Vec<i32>,
    }
    
    impl MySet {
        fn new() -> Self {
            MySet { data: Vec::new() }
        }
        
        fn insert(&mut self, value: i32) {
            self.data.push(value);
        }
        
        fn take<Q>(&mut self, value: &Q) -> Option<i32>
        where
            Q: ?Sized + std::hash::Hash + PartialEq<i32>,
        {
            self.swap_take(value)
        }

        fn swap_take<Q>(&mut self, value: &Q) -> Option<i32>
        where
            Q: ?Sized + PartialEq + std::hash::Hash,
        {
            if let Some(pos) = self.data.iter().position(|x| *x == *value) {
                let last = self.data.pop();
                if let Some(v) = last {
                    self.data.swap(pos, pos);
                }
                Some(value.clone())
            } else {
                None
            }
        }
    }

    let mut set = MySet::new();
    set.insert(1);
    set.insert(2);
    
    assert_eq!(set.take(&3), None);
    assert_eq!(set.take(&4), None);
}

#[test]
fn test_take_repeat_take() {
    struct MySet {
        data: Vec<i32>,
    }
    
    impl MySet {
        fn new() -> Self {
            MySet { data: Vec::new() }
        }
        
        fn insert(&mut self, value: i32) {
            self.data.push(value);
        }

        fn take<Q>(&mut self, value: &Q) -> Option<i32>
        where
            Q: ?Sized + std::hash::Hash + PartialEq<i32>,
        {
            self.swap_take(value)
        }

        fn swap_take<Q>(&mut self, value: &Q) -> Option<i32>
        where
            Q: ?Sized + PartialEq + std::hash::Hash,
        {
            if let Some(pos) = self.data.iter().position(|x| *x == *value) {
                let last = self.data.pop();
                if let Some(v) = last {
                    self.data.swap(pos, pos);
                }
                Some(value.clone())
            } else {
                None
            }
        }
    }
    
    let mut set = MySet::new();
    set.insert(1);
    set.insert(1);
    
    assert_eq!(set.take(&1), Some(1));
    assert_eq!(set.take(&1), Some(1));
    assert_eq!(set.take(&1), None);
}

