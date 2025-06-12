// Answer 0

#[test]
fn test_shrink_to_fit() {
    use std::collections::HashSet;

    struct MySet {
        map: HashSet<i32>,
    }

    impl MySet {
        pub fn new() -> Self {
            MySet {
                map: HashSet::new(),
            }
        }

        pub fn insert(&mut self, value: i32) {
            self.map.insert(value);
        }

        pub fn shrink_to_fit(&mut self) {
            self.map.shrink_to_fit();
        }

        pub fn capacity(&self) -> usize {
            self.map.capacity()
        }
    }

    let mut my_set = MySet::new();
    my_set.insert(1);
    my_set.insert(2);
    my_set.insert(3);

    // Initial capacity should be greater than or equal to the item count
    assert!(my_set.capacity() >= 3);

    // Call shrink_to_fit to reduce capacity
    my_set.shrink_to_fit();

    // After shrinking, the capacity should be equal to the item count
    assert_eq!(my_set.capacity(), 3);
}

