// Answer 0

#[test]
fn test_header_map_with_capacity_zero() {
    struct HeaderMap<T> {
        capacity: usize,
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> HeaderMap<T> {
        const MAX_SIZE: usize = 100; // Example maximum size
        pub fn try_with_capacity(capacity: usize) -> Result<Self, &'static str> {
            if capacity > Self::MAX_SIZE {
                Err("capacity exceeds MAX_SIZE")
            } else {
                Ok(Self {
                    capacity,
                    _marker: std::marker::PhantomData,
                })
            }
        }

        pub fn with_capacity(capacity: usize) -> HeaderMap<T> {
            Self::try_with_capacity(capacity).expect("size overflows MAX_SIZE")
        }

        pub fn is_empty(&self) -> bool {
            self.capacity == 0
        }
        
        pub fn capacity(&self) -> usize {
            self.capacity
        }
    }

    let map: HeaderMap<u32> = HeaderMap::with_capacity(0);
    assert!(map.is_empty());
    assert_eq!(0, map.capacity());
}

#[test]
fn test_header_map_with_capacity_below_max() {
    struct HeaderMap<T> {
        capacity: usize,
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> HeaderMap<T> {
        const MAX_SIZE: usize = 100; // Example maximum size
        pub fn try_with_capacity(capacity: usize) -> Result<Self, &'static str> {
            if capacity > Self::MAX_SIZE {
                Err("capacity exceeds MAX_SIZE")
            } else {
                Ok(Self {
                    capacity,
                    _marker: std::marker::PhantomData,
                })
            }
        }

        pub fn with_capacity(capacity: usize) -> HeaderMap<T> {
            Self::try_with_capacity(capacity).expect("size overflows MAX_SIZE")
        }

        pub fn is_empty(&self) -> bool {
            self.capacity == 0
        }
        
        pub fn capacity(&self) -> usize {
            self.capacity
        }
    }

    let map: HeaderMap<u32> = HeaderMap::with_capacity(10);
    assert!(map.is_empty());
    assert_eq!(10, map.capacity());
}

#[test]
#[should_panic(expected = "size overflows MAX_SIZE")]
fn test_header_map_with_capacity_exceeds_max() {
    struct HeaderMap<T> {
        capacity: usize,
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> HeaderMap<T> {
        const MAX_SIZE: usize = 100; // Example maximum size
        pub fn try_with_capacity(capacity: usize) -> Result<Self, &'static str> {
            if capacity > Self::MAX_SIZE {
                Err("capacity exceeds MAX_SIZE")
            } else {
                Ok(Self {
                    capacity,
                    _marker: std::marker::PhantomData,
                })
            }
        }

        pub fn with_capacity(capacity: usize) -> HeaderMap<T> {
            Self::try_with_capacity(capacity).expect("size overflows MAX_SIZE")
        }

        pub fn is_empty(&self) -> bool {
            self.capacity == 0
        }
        
        pub fn capacity(&self) -> usize {
            self.capacity
        }
    }

    let _ = HeaderMap::with_capacity(101); // Should panic
}

