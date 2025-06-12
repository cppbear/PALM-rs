// Answer 0

#[test]
fn test_try_insert_phase_two_success_with_danger() {
    struct TestStruct {
        entries: Vec<(HeaderName, T)>,
        indices: Vec<Pos>,
        danger: DangerLevel,
    }

    impl TestStruct {
        fn try_insert_entry(&mut self, hash: HashValue, key: HeaderName, value: T) -> Result<(), MaxSizeReached> {
            self.entries.push((key, value));
            Ok(())
        }
    }

    let mut test_instance = TestStruct {
        entries: vec![],
        indices: vec![],
        danger: DangerLevel::new(),
    };

    let key = HeaderName::new("Test-Header");
    let value = T::new("TestValue");
    let hash = HashValue::new(12345);
    let probe = 1;
    let danger = true;

    let result = test_instance.try_insert_phase_two(key, value, hash, probe, danger);
    
    assert_eq!(result, Ok(0));
    assert_eq!(test_instance.danger.level, DangerLevel::YELLOW);
}

#[test]
fn test_try_insert_phase_two_no_danger() {
    struct TestStruct {
        entries: Vec<(HeaderName, T)>,
        indices: Vec<Pos>,
        danger: DangerLevel,
    }

    impl TestStruct {
        fn try_insert_entry(&mut self, hash: HashValue, key: HeaderName, value: T) -> Result<(), MaxSizeReached> {
            self.entries.push((key, value));
            Ok(())
        }
    }

    let mut test_instance = TestStruct {
        entries: vec![],
        indices: vec![],
        danger: DangerLevel::new(),
    };

    let key = HeaderName::new("Test-Header");
    let value = T::new("TestValue");
    let hash = HashValue::new(12345);
    let probe = 1;
    let danger = false;

    let result = test_instance.try_insert_phase_two(key, value, hash, probe, danger);
    
    assert_eq!(result, Ok(0));
    assert_eq!(test_instance.danger.level, DangerLevel::GREEN);
}

