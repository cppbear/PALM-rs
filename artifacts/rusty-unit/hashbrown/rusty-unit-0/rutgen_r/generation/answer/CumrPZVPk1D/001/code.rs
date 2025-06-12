// Answer 0

#[test]
fn test_iter_non_empty() {
    struct Raw {
        iter: Vec<i32>,
    }

    impl Raw {
        pub fn iter(&self) -> Vec<i32> {
            self.iter.clone()
        }
    }

    let raw_instance = Raw {
        iter: vec![1, 2, 3, 4, 5],
    };

    let result = raw_instance.iter();
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_iter_empty() {
    struct Raw {
        iter: Vec<i32>,
    }

    impl Raw {
        pub fn iter(&self) -> Vec<i32> {
            self.iter.clone()
        }
    }

    let raw_instance = Raw {
        iter: Vec::new(),
    };

    let result = raw_instance.iter();
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_iter_large_data() {
    struct Raw {
        iter: Vec<i32>,
    }

    impl Raw {
        pub fn iter(&self) -> Vec<i32> {
            self.iter.clone()
        }
    }

    let raw_instance = Raw {
        iter: (0..1000).collect(),
    };

    let result = raw_instance.iter();
    assert_eq!(result.len(), 1000);
    assert_eq!(result[0], 0);
    assert_eq!(result[999], 999);
}

