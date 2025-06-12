// Answer 0

#[test]
#[should_panic(expected = "Hash table capacity overflow")]
fn test_new_uninitialized_buckets_not_power_of_two_1() {
    let allocator = Global; 
    let table_layout = TableLayout::new::<u8>();
    let buckets = 3;
    let fallibility = Fallibility::Infallible;
    unsafe { RawTableInner::new_uninitialized(&allocator, table_layout, buckets, fallibility) };
}

#[test]
#[should_panic(expected = "Hash table capacity overflow")]
fn test_new_uninitialized_buckets_not_power_of_two_2() {
    let allocator = Global; 
    let table_layout = TableLayout::new::<u8>();
    let buckets = 5;
    let fallibility = Fallibility::Infallible;
    unsafe { RawTableInner::new_uninitialized(&allocator, table_layout, buckets, fallibility) };
}

#[test]
#[should_panic(expected = "Hash table capacity overflow")]
fn test_new_uninitialized_buckets_not_power_of_two_3() {
    let allocator = Global; 
    let table_layout = TableLayout::new::<u8>();
    let buckets = 6;
    let fallibility = Fallibility::Infallible;
    unsafe { RawTableInner::new_uninitialized(&allocator, table_layout, buckets, fallibility) };
}

#[test]
#[should_panic(expected = "Hash table capacity overflow")]
fn test_new_uninitialized_buckets_not_power_of_two_4() {
    let allocator = Global; 
    let table_layout = TableLayout::new::<u8>();
    let buckets = 7;
    let fallibility = Fallibility::Infallible;
    unsafe { RawTableInner::new_uninitialized(&allocator, table_layout, buckets, fallibility) };
}

#[test]
#[should_panic(expected = "Hash table capacity overflow")]
fn test_new_uninitialized_buckets_not_power_of_two_5() {
    let allocator = Global; 
    let table_layout = TableLayout::new::<u8>();
    let buckets = 9;
    let fallibility = Fallibility::Infallible;
    unsafe { RawTableInner::new_uninitialized(&allocator, table_layout, buckets, fallibility) };
}

#[test]
#[should_panic(expected = "Hash table capacity overflow")]
fn test_new_uninitialized_buckets_not_power_of_two_6() {
    let allocator = Global; 
    let table_layout = TableLayout::new::<u8>();
    let buckets = 10;
    let fallibility = Fallibility::Infallible;
    unsafe { RawTableInner::new_uninitialized(&allocator, table_layout, buckets, fallibility) };
}

