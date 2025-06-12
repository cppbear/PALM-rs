// Answer 0

#[derive(Debug)]
struct TestMap {
    map: Vec<i32>,
}

#[derive(Debug)]
struct TryReserveError;

impl TestMap {
    pub fn new() -> Self {
        TestMap { map: Vec::new() }
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        if additional + self.map.len() > 100 {
            Err(TryReserveError)
        } else {
            self.map.resize(self.map.len() + additional, 0);
            Ok(())
        }
    }
}

#[test]
fn test_try_reserve_success() {
    let mut test_map = TestMap::new();
    let result = test_map.try_reserve(10);
    assert!(result.is_ok());
    assert_eq!(test_map.map.len(), 10);
}

#[test]
fn test_try_reserve_boundary() {
    let mut test_map = TestMap::new();
    let result = test_map.try_reserve(100);
    assert!(result.is_ok());
    assert_eq!(test_map.map.len(), 100);
}

#[test]
#[should_panic]
fn test_try_reserve_exceed_capacity() {
    let mut test_map = TestMap::new();
    let _ = test_map.try_reserve(101);
}

