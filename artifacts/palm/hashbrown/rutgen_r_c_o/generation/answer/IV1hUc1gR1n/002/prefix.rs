// Answer 0

#[test]
fn test_reserve_rehash_inner_should_panic_overflow() {
    let alloc = &Global;
    let layout = TableLayout { size: 8, ctrl_align: 8 };
    let mut table = RawTableInner::with_capacity(alloc, layout, usize::MAX);
    let additional = 1;
    
    unsafe {
        table.reserve_rehash_inner(alloc, additional, &|_, _| 0, Fallibility::Infallible, layout, None);
    }
}

#[test]
fn test_reserve_rehash_inner_with_full_capacity() {
    let alloc = &Global;
    let layout = TableLayout { size: 8, ctrl_align: 8 };
    let capacity = 16;
    let mut table = RawTableInner::with_capacity(alloc, layout, capacity);
    table.items = capacity - 1;
    let additional = 2;

    unsafe {
        table.reserve_rehash_inner(alloc, additional, &|_, _| 0, Fallibility::Fallible, layout, None);
    }
}

#[test]
fn test_reserve_rehash_inner_below_half_capacity() {
    let alloc = &Global;
    let layout = TableLayout { size: 8, ctrl_align: 8 };
    let capacity = 32;
    let mut table = RawTableInner::with_capacity(alloc, layout, capacity);
    table.items = 12; // less than half of capacity
    let additional = 2;

    unsafe {
        table.reserve_rehash_inner(alloc, additional, &|_, _| 0, Fallibility::Infallible, layout, None);
    }
}

#[test]
fn test_reserve_rehash_inner_above_half_capacity() {
    let alloc = &Global;
    let layout = TableLayout { size: 8, ctrl_align: 8 };
    let capacity = 32;
    let mut table = RawTableInner::with_capacity(alloc, layout, capacity);
    table.items = 16; // exactly half of capacity
    let additional = 3;

    unsafe {
        table.reserve_rehash_inner(alloc, additional, &|_, _| 0, Fallibility::Fallible, layout, None);
    }
}

#[test]
fn test_reserve_rehash_inner_edge_case_items_equal_max() {
    let alloc = &Global;
    let layout = TableLayout { size: 8, ctrl_align: 8 };
    let capacity = usize::MAX; // extreme case
    let mut table = RawTableInner::with_capacity(alloc, layout, capacity);
    table.items = usize::MAX - 1; // almost max size
    let additional = 1; // to test overflow

    unsafe {
        let result = table.reserve_rehash_inner(alloc, additional, &|_, _| 0, Fallibility::Infallible, layout, None);
        assert!(result.is_err());
    }
}

