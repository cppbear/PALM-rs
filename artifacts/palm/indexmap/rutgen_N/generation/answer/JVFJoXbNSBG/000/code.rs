// Answer 0

#[test]
fn test_split_off_valid_range() {
    use std::collections::HashMap;

    struct TestMap {
        core: HashMap<i32, i32>,
        hash_builder: std::collections::hash_map::RandomState,
    }

    impl TestMap {
        fn new() -> Self {
            let mut core = HashMap::new();
            for i in 0..5 {
                core.insert(i, i * 10);
            }
            Self {
                core,
                hash_builder: std::collections::hash_map::RandomState::new(),
            }
        }

        fn split_off(&mut self, at: usize) -> Self {
            if at > self.core.len() {
                panic!("at cannot be greater than len");
            }
            let split_core: HashMap<_, _> = self.core.clone().iter().skip(at).map(|(&k, &v)| (k, v)).collect();
            self.core = self.core.clone().iter().take(at).map(|(&k, &v)| (k, v)).collect();
            Self {
                core: split_core,
                hash_builder: self.hash_builder.clone(),
            }
        }
    }

    let mut map = TestMap::new();
    let split_map = map.split_off(3);
    assert_eq!(map.core.len(), 3);
    assert_eq!(split_map.core.len(), 2);
    assert_eq!(split_map.core.get(&3), Some(&30));
    assert_eq!(split_map.core.get(&4), Some(&40));
}

#[test]
#[should_panic(expected = "at cannot be greater than len")]
fn test_split_off_over_flow() {
    use std::collections::HashMap;

    struct TestMap {
        core: HashMap<i32, i32>,
        hash_builder: std::collections::hash_map::RandomState,
    }

    impl TestMap {
        fn new() -> Self {
            let mut core = HashMap::new();
            for i in 0..5 {
                core.insert(i, i * 10);
            }
            Self {
                core,
                hash_builder: std::collections::hash_map::RandomState::new(),
            }
        }

        fn split_off(&mut self, at: usize) -> Self {
            if at > self.core.len() {
                panic!("at cannot be greater than len");
            }
            let split_core: HashMap<_, _> = self.core.clone().iter().skip(at).map(|(&k, &v)| (k, v)).collect();
            self.core = self.core.clone().iter().take(at).map(|(&k, &v)| (k, v)).collect();
            Self {
                core: split_core,
                hash_builder: self.hash_builder.clone(),
            }
        }
    }

    let mut map = TestMap::new();
    let _ = map.split_off(6);
}

