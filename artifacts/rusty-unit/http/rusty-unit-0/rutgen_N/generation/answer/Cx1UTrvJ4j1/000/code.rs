// Answer 0

#[test]
fn test_reserve_positive_capacity() {
    struct HeaderMap {
        capacity: usize,
        max_size: usize,
    }

    impl HeaderMap {
        const MAX_SIZE: usize = 100;

        pub fn new() -> Self {
            HeaderMap { capacity: 0, max_size: Self::MAX_SIZE }
        }

        pub fn try_reserve(&mut self, additional: usize) -> Result<(), ()> {
            if self.capacity + additional > self.max_size {
                Err(())
            } else {
                self.capacity += additional;
                Ok(())
            }
        }

        pub fn reserve(&mut self, additional: usize) {
            self.try_reserve(additional).expect("size overflows MAX_SIZE")
        }
    }

    let mut map = HeaderMap::new();
    map.reserve(10);
    assert_eq!(map.capacity, 10);
}

#[test]
fn test_reserve_boundary_capacity() {
    struct HeaderMap {
        capacity: usize,
        max_size: usize,
    }

    impl HeaderMap {
        const MAX_SIZE: usize = 100;

        pub fn new() -> Self {
            HeaderMap { capacity: 0, max_size: Self::MAX_SIZE }
        }

        pub fn try_reserve(&mut self, additional: usize) -> Result<(), ()> {
            if self.capacity + additional > self.max_size {
                Err(())
            } else {
                self.capacity += additional;
                Ok(())
            }
        }

        pub fn reserve(&mut self, additional: usize) {
            self.try_reserve(additional).expect("size overflows MAX_SIZE")
        }
    }

    let mut map = HeaderMap::new();
    map.reserve(100);
    assert_eq!(map.capacity, 100);
}

#[should_panic]
#[test]
fn test_reserve_exceeding_capacity() {
    struct HeaderMap {
        capacity: usize,
        max_size: usize,
    }

    impl HeaderMap {
        const MAX_SIZE: usize = 100;

        pub fn new() -> Self {
            HeaderMap { capacity: 0, max_size: Self::MAX_SIZE }
        }

        pub fn try_reserve(&mut self, additional: usize) -> Result<(), ()> {
            if self.capacity + additional > self.max_size {
                Err(())
            } else {
                self.capacity += additional;
                Ok(())
            }
        }

        pub fn reserve(&mut self, additional: usize) {
            self.try_reserve(additional).expect("size overflows MAX_SIZE")
        }
    }

    let mut map = HeaderMap::new();
    map.reserve(101);
}

