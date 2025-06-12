// Answer 0

#[test]
fn test_replace_full_existing_value() {
    struct TestSet<T> {
        map: TestMap<T>,
    }

    struct TestMap<T> {
        core: TestCore<T>,
    }

    struct TestCore<T> {
        values: Vec<T>,
    }

    impl<T: PartialEq> TestCore<T> {
        fn replace_full(&mut self, _hash: usize, value: T, _: ()) -> (usize, Option<(T, ())>) {
            if let Some(pos) = self.values.iter().position(|x| *x == value) {
                let replaced = self.values[pos];
                self.values[pos] = value;
                (pos, Some((replaced, ())))
            } else {
                self.values.push(value);
                (self.values.len() - 1, None)
            }
        }
    }

    let mut set = TestSet {
        map: TestMap {
            core: TestCore { values: vec!["existing_value"] },
        },
    };

    let (index, replaced) = set.replace_full("existing_value");
    assert_eq!(index, 0);
    assert_eq!(replaced, Some("existing_value"));
}

#[test]
fn test_replace_full_new_value() {
    struct TestSet<T> {
        map: TestMap<T>,
    }

    struct TestMap<T> {
        core: TestCore<T>,
    }

    struct TestCore<T> {
        values: Vec<T>,
    }

    impl<T: PartialEq> TestCore<T> {
        fn replace_full(&mut self, _hash: usize, value: T, _: ()) -> (usize, Option<(T, ())>) {
            if let Some(pos) = self.values.iter().position(|x| *x == value) {
                let replaced = self.values[pos];
                self.values[pos] = value;
                (pos, Some((replaced, ())))
            } else {
                self.values.push(value);
                (self.values.len() - 1, None)
            }
        }
    }

    let mut set = TestSet {
        map: TestMap {
            core: TestCore { values: vec![] },
        },
    };

    let (index, replaced) = set.replace_full("new_value");
    assert_eq!(index, 0);
    assert_eq!(replaced, None);
}

#[test]
fn test_replace_full_multiple_values() {
    struct TestSet<T> {
        map: TestMap<T>,
    }

    struct TestMap<T> {
        core: TestCore<T>,
    }

    struct TestCore<T> {
        values: Vec<T>,
    }

    impl<T: PartialEq> TestCore<T> {
        fn replace_full(&mut self, _hash: usize, value: T, _: ()) -> (usize, Option<(T, ())>) {
            if let Some(pos) = self.values.iter().position(|x| *x == value) {
                let replaced = self.values[pos];
                self.values[pos] = value;
                (pos, Some((replaced, ())))
            } else {
                self.values.push(value);
                (self.values.len() - 1, None)
            }
        }
    }

    let mut set = TestSet {
        map: TestMap {
            core: TestCore { values: vec!["value1", "value2"] },
        },
    };

    let (index, replaced) = set.replace_full("value1");
    assert_eq!(index, 0);
    assert_eq!(replaced, Some("value1"));
    
    let (index_new, replaced_new) = set.replace_full("value3");
    assert_eq!(index_new, 2);
    assert_eq!(replaced_new, None);
}

