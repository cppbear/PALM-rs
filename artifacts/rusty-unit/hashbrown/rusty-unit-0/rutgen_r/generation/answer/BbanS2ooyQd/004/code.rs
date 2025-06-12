// Answer 0

#[test]
fn test_shrink_to_zero() {
    struct TestAllocator;

    struct RawTable<T> {
        table: RawTableInner<T>,
        buckets: usize,
        alloc: TestAllocator,
    }

    struct RawTableInner<T> {
        items: usize,
    }

    impl RawTableInner<u64> {
        const NEW: Self = Self { items: 0 };
        
        unsafe fn drop_inner_table<T, A>(&mut self, _alloc: &A, _layout: usize) {
            // Simulating the drop logic, does nothing for testing purposes.
        }
        
        fn with_capacity<A>(_alloc: &A, _layout: usize, _min_size: usize) -> Self {
            Self { items: 0 }
        }
    }

    impl RawTable<u64> {
        fn buckets(&self) -> usize {
            self.buckets
        }
        
        // Mocking the resize function which has to simulate successful completion.
        unsafe fn resize(
            &mut self,
            _min_size: usize,
            _hasher: impl Fn(&u64) -> u64,
            _fallibility: Fallibility,
        ) -> Result<(), ()> {
            Ok(())
        }

        pub fn shrink_to(&mut self, min_size: usize, hasher: impl Fn(&u64) -> u64) {
            let min_size = usize::max(self.table.items, min_size);
            if min_size == 0 {
                let mut old_inner = std::mem::replace(&mut self.table, RawTableInner::NEW);
                unsafe {
                    old_inner.drop_inner_table::<u64, _>(&self.alloc, 0);
                }
                return;
            }

            let min_buckets = match capacity_to_buckets(min_size) {
                Some(buckets) => buckets,
                None => return,
            };

            if min_buckets < self.buckets() {
                if self.table.items == 0 {
                    let new_inner = RawTableInner::with_capacity(&self.alloc, 0, min_size);
                    let mut old_inner = std::mem::replace(&mut self.table, new_inner);
                    unsafe {
                        old_inner.drop_inner_table::<u64, _>(&self.alloc, 0);
                    }
                } else {
                    unsafe {
                        if self.resize(min_size, hasher, Fallibility::Infallible).is_err() {
                            std::hint::unreachable_unchecked()
                        }
                    }
                }
            }
        }
    }

    enum Fallibility {
        Infallible,
    }

    fn capacity_to_buckets(size: usize) -> Option<usize> {
        Some(size.next_power_of_two())
    }

    let mut table: RawTable<u64> = RawTable {
        table: RawTableInner { items: 0 },
        buckets: 8, // Arbitrary initial bucket count
        alloc: TestAllocator,
    };

    table.shrink_to(0, |x| *x as u64);
}

