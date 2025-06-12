// Answer 0

#[test]
fn test_new_uninitialized_buckets_not_power_of_two_3() {
    let alloc = Global;
    let buckets = 3;
    let fallibility = Fallibility::Fallible;
    unsafe {
        let result = RawTable::new_uninitialized(alloc, buckets, fallibility);
    }
}

#[test]
fn test_new_uninitialized_buckets_not_power_of_two_5() {
    let alloc = Global;
    let buckets = 5;
    let fallibility = Fallibility::Fallible;
    unsafe {
        let result = RawTable::new_uninitialized(alloc, buckets, fallibility);
    }
}

#[test]
fn test_new_uninitialized_buckets_not_power_of_two_6() {
    let alloc = Global;
    let buckets = 6;
    let fallibility = Fallibility::Fallible;
    unsafe {
        let result = RawTable::new_uninitialized(alloc, buckets, fallibility);
    }
}

#[test]
fn test_new_uninitialized_buckets_not_power_of_two_7() {
    let alloc = Global;
    let buckets = 7;
    let fallibility = Fallibility::Fallible;
    unsafe {
        let result = RawTable::new_uninitialized(alloc, buckets, fallibility);
    }
}

#[test]
fn test_new_uninitialized_buckets_not_power_of_two_10() {
    let alloc = Global;
    let buckets = 10;
    let fallibility = Fallibility::Fallible;
    unsafe {
        let result = RawTable::new_uninitialized(alloc, buckets, fallibility);
    }
}

