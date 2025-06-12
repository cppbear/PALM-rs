// Answer 0

#[test]
fn test_get_index_valid() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            TestStruct { data }
        }

        fn as_entries(&self) -> Vec<Bucket<i32>> {
            self.data.iter().map(|&x| Bucket { key: x }).collect()
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn get_index(&self, index: usize) -> Option<&i32> {
            self.as_entries().get(index).map(Bucket::key_ref)
        }
    }

    struct Bucket<T> {
        key: T,
    }

    impl<T> Bucket<T> {
        fn key_ref(&self) -> &T {
            &self.key
        }
    }

    let test_data = TestStruct::new(vec![10, 20, 30, 40, 50]);
    assert_eq!(test_data.get_index(0), Some(&10));
    assert_eq!(test_data.get_index(1), Some(&20));
    assert_eq!(test_data.get_index(2), Some(&30));
    assert_eq!(test_data.get_index(3), Some(&40));
    assert_eq!(test_data.get_index(4), Some(&50));
}

#[test]
fn test_get_index_out_of_bounds() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            TestStruct { data }
        }

        fn as_entries(&self) -> Vec<Bucket<i32>> {
            self.data.iter().map(|&x| Bucket { key: x }).collect()
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn get_index(&self, index: usize) -> Option<&i32> {
            self.as_entries().get(index).map(Bucket::key_ref)
        }
    }

    struct Bucket<T> {
        key: T,
    }

    impl<T> Bucket<T> {
        fn key_ref(&self) -> &T {
            &self.key
        }
    }

    let test_data = TestStruct::new(vec![1, 2, 3]);
    assert_eq!(test_data.get_index(3), None);
    assert_eq!(test_data.get_index(100), None);
}

#[test]
fn test_get_index_empty() {
    struct TestStruct {
        data: Vec<i32>,
    }

    impl TestStruct {
        fn new(data: Vec<i32>) -> Self {
            TestStruct { data }
        }

        fn as_entries(&self) -> Vec<Bucket<i32>> {
            self.data.iter().map(|&x| Bucket { key: x }).collect()
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn get_index(&self, index: usize) -> Option<&i32> {
            self.as_entries().get(index).map(Bucket::key_ref)
        }
    }

    struct Bucket<T> {
        key: T,
    }

    impl<T> Bucket<T> {
        fn key_ref(&self) -> &T {
            &self.key
        }
    }

    let test_data = TestStruct::new(vec![]);
    assert_eq!(test_data.get_index(0), None);
}

