// Answer 0

#[test]
fn test_and_modify_with_occupied_entry() {
    struct Entry<V> {
        occupied: bool,
        value: Option<V>,
    }

    impl<V> Entry<V> {
        fn get_mut(&mut self) -> &mut V {
            self.value.as_mut().expect("Entry is not occupied")
        }
    }

    impl<V> Entry<V> {
        fn and_modify<F>(mut self, f: F) -> Self
        where
            F: FnOnce(&mut V),
        {
            if self.occupied {
                f(self.get_mut());
            }
            self
        }
    }

    let mut entry = Entry::<i32> {
        occupied: true,
        value: Some(10),
    };

    entry = entry.and_modify(|v| *v += 5);
    
    assert_eq!(entry.value, Some(15));
}

#[test]
#[should_panic]
fn test_and_modify_with_unoccupied_entry() {
    struct Entry<V> {
        occupied: bool,
        value: Option<V>,
    }

    impl<V> Entry<V> {
        fn get_mut(&mut self) -> &mut V {
            self.value.as_mut().expect("Entry is not occupied")
        }
    }

    impl<V> Entry<V> {
        fn and_modify<F>(mut self, f: F) -> Self
        where
            F: FnOnce(&mut V),
        {
            if self.occupied {
                f(self.get_mut());
            }
            self
        }
    }

    let mut entry = Entry::<i32> {
        occupied: false,
        value: None,
    };

    entry.and_modify(|_v| panic!("This should panic"));
}

