// Answer 0

#[test]
fn test_reserve_rehash_with_drop() {
    struct TestStruct {
        value: i32,
    }

    impl TestStruct {
        const NEEDS_DROP: bool = true;
    }

    struct RawTableInner {
        table: Vec<Option<TestStruct>>,
        alloc: (),
    }

    impl RawTableInner {
        fn reserve_rehash_inner(
            &mut self,
            _alloc: &(),
            additional: usize,
            hasher: &dyn Fn(&RawTableInner, usize) -> u64,
            _fallibility: Fallibility,
            _layout: (),
            drop_fn: Option<fn(*mut TestStruct)>,
        ) -> Result<(), TryReserveError> {
            self.table.resize(self.table.len() + additional, None);
            drop_fn.map(|f| f(self.table.as_mut_ptr() as *mut TestStruct));
            Ok(())
        }
        
        unsafe fn reserve_rehash(
            &mut self,
            additional: usize,
            hasher: impl Fn(&TestStruct) -> u64,
            fallibility: Fallibility,
        ) -> Result<(), TryReserveError> {
            unsafe {
                self.reserve_rehash_inner(
                    &self.alloc,
                    additional,
                    &|table, index| hasher(table.bucket::<TestStruct>(index).as_ref().unwrap()),
                    fallibility,
                    (),
                    Some(|ptr| ptr::drop_in_place(ptr as *mut TestStruct)),
                )
            }
        }

        fn bucket<T>(&self, index: usize) -> &Option<T> {
            &self.table[index]
        }
    }

    let mut raw_table = RawTableInner {
        table: Vec::new(),
        alloc: (),
    };
    
    let additional = 10;
    let hasher = |item: &TestStruct| item.value as u64;

    let result = unsafe { raw_table.reserve_rehash(additional, hasher, Fallibility::All) };
    
    assert!(result.is_ok());
    assert_eq!(raw_table.table.len(), additional);
}

