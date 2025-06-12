// Answer 0

#[test]
fn test_prepare_resize_ok() {
    struct TestAllocator;
    struct TestTableLayout;

    impl Allocator for TestAllocator {
        // Implement required methods for the Allocator trait here
    }

    let alloc = TestAllocator;
    let table_layout = TestTableLayout;
    let capacity = 10;
    let fallibility = Fallibility::All;

    struct RawTable {
        items: usize,
    }

    impl RawTable {
        fn prepare_resize<'a, A>(
            &self,
            alloc: &'a A,
            table_layout: TableLayout,
            capacity: usize,
            fallibility: Fallibility,
        ) -> Result<crate::scopeguard::ScopeGuard<Self, impl FnMut(&mut Self) + 'a>, TryReserveError>
        where
            A: Allocator,
        {
            debug_assert!(self.items <= capacity);
            let new_table = RawTableInner::fallible_with_capacity(alloc, table_layout, capacity, fallibility)?;
            Ok(guard(new_table, move |self_| {
                if !self_.is_empty_singleton() {
                    unsafe { self_.free_buckets(alloc, table_layout) };
                }
            }))
        }
    }

    let table = RawTable { items: 10 };
    let result = table.prepare_resize(&alloc, table_layout, capacity, fallibility);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_prepare_resize_panics_on_items_exceed_capacity() {
    struct TestAllocator;
    struct TestTableLayout;

    impl Allocator for TestAllocator {
        // Implement required methods for the Allocator trait here
    }

    let alloc = TestAllocator;
    let table_layout = TestTableLayout;
    let capacity = 10;
    let fallibility = Fallibility::All;

    struct RawTable {
        items: usize,
    }

    impl RawTable {
        fn prepare_resize<'a, A>(
            &self,
            alloc: &'a A,
            table_layout: TableLayout,
            capacity: usize,
            fallibility: Fallibility,
        ) -> Result<crate::scopeguard::ScopeGuard<Self, impl FnMut(&mut Self) + 'a>, TryReserveError>
        where
            A: Allocator,
        {
            debug_assert!(self.items <= capacity);
            let new_table = RawTableInner::fallible_with_capacity(alloc, table_layout, capacity, fallibility)?;
            Ok(guard(new_table, move |self_| {
                if !self_.is_empty_singleton() {
                    unsafe { self_.free_buckets(alloc, table_layout) };
                }
            }))
        }
    }

    let table = RawTable { items: 11 };
    let _ = table.prepare_resize(&alloc, table_layout, capacity, fallibility);
}

