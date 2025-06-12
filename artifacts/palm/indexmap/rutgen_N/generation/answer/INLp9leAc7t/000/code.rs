// Answer 0

#[derive(Debug)]
struct Core {
    capacity: usize,
    reserved: usize,
}

#[derive(Debug)]
struct MyMap {
    core: Core,
}

#[derive(Debug)]
struct TryReserveError;

impl Core {
    fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        if self.reserved + additional > self.capacity {
            Err(TryReserveError)
        } else {
            self.reserved += additional;
            Ok(())
        }
    }
}

impl MyMap {
    pub fn new(capacity: usize) -> Self {
        MyMap {
            core: Core { capacity, reserved: 0 },
        }
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.core.try_reserve(additional)
    }
}

#[test]
fn test_try_reserve_success() {
    let mut map = MyMap::new(10);
    let result = map.try_reserve(5);
    assert!(result.is_ok());
    assert_eq!(map.core.reserved, 5);
}

#[test]
fn test_try_reserve_exceed_capacity() {
    let mut map = MyMap::new(10);
    let _ = map.try_reserve(10).unwrap(); // Reserve 10
    let result = map.try_reserve(1); // Try to reserve 1 more
    assert!(result.is_err());
}

#[test]
fn test_try_reserve_exact_capacity() {
    let mut map = MyMap::new(10);
    let _ = map.try_reserve(10).unwrap(); // Reserve exactly 10
    let result = map.try_reserve(0); // Try to reserve 0 more
    assert!(result.is_ok());
    assert_eq!(map.core.reserved, 10);
}

