// Answer 0

#[test]
fn test_reserve_success_case_min() {
    let mut table = RawTable::new_in(Global);
    table.table.growth_left = 2; 
    table.reserve(1, |x| x as u64);
}

#[test]
fn test_reserve_success_case_max() {
    let mut table = RawTable::new_in(Global);
    table.table.growth_left = 10; 
    table.reserve(10, |x| x as u64);
}

#[test]
fn test_reserve_success_case_edge() {
    let mut table = RawTable::new_in(Global);
    table.table.growth_left = 5; 
    table.reserve(5, |x| x as u64);
}

#[test]
#[should_panic]
fn test_reserve_failure_case_overflow() {
    let mut table = RawTable::new_in(Global);
    table.table.growth_left = 1; 
    table.reserve(2, |x| x as u64);
}

#[test]
fn test_reserve_success_case_rehash() {
    let mut table = RawTable::new_in(Global);
    table.table.growth_left = 5; 
    table.reserve(3, |x| x as u64);
}

