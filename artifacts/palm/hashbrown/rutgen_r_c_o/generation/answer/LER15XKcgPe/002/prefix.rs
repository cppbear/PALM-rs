// Answer 0

#[test]
fn test_prepare_rehash_in_place_with_zero_buckets() {
    let mut raw_table = RawTableInner::with_capacity(&Global, TableLayout::default(), 0);
    unsafe {
        raw_table.prepare_rehash_in_place();
    }
}

#[test]
fn test_prepare_rehash_in_place_with_exact_group_width() {
    let group_width = Group::WIDTH;
    let mut raw_table = RawTableInner::with_capacity(&Global, TableLayout::default(), group_width);
    unsafe {
        raw_table.prepare_rehash_in_place();
    }
}

#[test]
fn test_prepare_rehash_in_place_with_more_than_group_width() {
    let group_width = Group::WIDTH + 1;
    let mut raw_table = RawTableInner::with_capacity(&Global, TableLayout::default(), group_width);
    unsafe {
        raw_table.prepare_rehash_in_place();
    }
}

#[test]
#[should_panic]
fn test_prepare_rehash_in_place_with_buckets_less_than_group_width() {
    let group_width = Group::WIDTH;
    let mut raw_table = RawTableInner::with_capacity(&Global, TableLayout::default(), group_width - 1);
    unsafe {
        raw_table.prepare_rehash_in_place();
    }
}

