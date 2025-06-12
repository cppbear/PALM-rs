// Answer 0

#[test]
fn test_try_insert_phase_two_success() {
    struct MockHeaderName;
    struct MockHashValue(usize);
    struct MockMaxSizeReached;

    impl MockHeaderName {
        fn new() -> Self {
            MockHeaderName
        }
    }

    impl MockHashValue {
        fn new(value: usize) -> Self {
            MockHashValue(value)
        }
    }
    
    struct TestStruct<T> {
        entries: Vec<T>,
        indices: Vec<usize>,
        danger: DangerLevel,
    }

    impl<T> TestStruct<T> {
        fn new() -> Self {
            TestStruct {
                entries: Vec::new(),
                indices: Vec::new(),
                danger: DangerLevel::Normal,
            }
        }

        fn try_insert_entry(&mut self, _hash: MockHashValue, _key: MockHeaderName, value: T) -> Result<(), MockMaxSizeReached> {
            self.entries.push(value);
            Ok(())
        }
    }

    enum DangerLevel {
        Normal,
        Yellow,
    }

    impl DangerLevel {
        fn set_yellow(&mut self) {
            *self = DangerLevel::Yellow;
        }
    }

    let mut test_struct: TestStruct<i32> = TestStruct::new();
    let key = MockHeaderName::new();
    let value = 42;
    let hash = MockHashValue::new(1);
    let probe = 0;
    let danger = false;

    let result = test_struct.try_insert_phase_two(key, value, hash, probe, danger);
    
    assert_eq!(result.is_ok(), true);
    assert_eq!(test_struct.entries.len(), 1);
    assert_eq!(test_struct.entries[0], 42);
    assert_eq!(test_struct.danger, DangerLevel::Normal);
}

#[test]
fn test_try_insert_phase_two_with_danger() {
    struct MockHeaderName;
    struct MockHashValue(usize);
    struct MockMaxSizeReached;

    impl MockHeaderName {
        fn new() -> Self {
            MockHeaderName
        }
    }

    impl MockHashValue {
        fn new(value: usize) -> Self {
            MockHashValue(value)
        }
    }
    
    struct TestStruct<T> {
        entries: Vec<T>,
        indices: Vec<usize>,
        danger: DangerLevel,
    }

    impl<T> TestStruct<T> {
        fn new() -> Self {
            TestStruct {
                entries: Vec::new(),
                indices: Vec::new(),
                danger: DangerLevel::Normal,
            }
        }

        fn try_insert_entry(&mut self, _hash: MockHashValue, _key: MockHeaderName, value: T) -> Result<(), MockMaxSizeReached> {
            self.entries.push(value);
            Ok(())
        }
    }

    enum DangerLevel {
        Normal,
        Yellow,
    }

    impl DangerLevel {
        fn set_yellow(&mut self) {
            *self = DangerLevel::Yellow;
        }
    }

    fn do_insert_phase_two(indices: &mut Vec<usize>, probe: usize, pos: Pos) -> usize {
        indices.push(probe); // Mock behavior
        1 // Return number of displaced
    }

    struct Pos {
        index: usize,
        hash: MockHashValue,
    }

    impl Pos {
        fn new(index: usize, hash: MockHashValue) -> Self {
            Pos { index, hash }
        }
    }

    let mut test_struct: TestStruct<i32> = TestStruct::new();
    let key = MockHeaderName::new();
    let value = 42;
    let hash = MockHashValue::new(1);
    let probe = 0;
    let danger = true;

    let result = test_struct.try_insert_phase_two(key, value, hash, probe, danger);
    
    assert_eq!(result.is_ok(), true);
    assert_eq!(test_struct.entries.len(), 1);
    assert_eq!(test_struct.entries[0], 42);
    assert_eq!(test_struct.danger, DangerLevel::Yellow);
}

