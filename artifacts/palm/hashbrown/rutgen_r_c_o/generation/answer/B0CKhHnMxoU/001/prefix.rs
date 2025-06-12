// Answer 0

#[test]
fn test_with_capacity_zero() {
    let table: HashTable<&str> = HashTable::with_capacity(0);
}

#[test]
fn test_with_capacity_one() {
    let table: HashTable<&str> = HashTable::with_capacity(1);
}

#[test]
fn test_with_capacity_ten() {
    let table: HashTable<&str> = HashTable::with_capacity(10);
}

#[test]
fn test_with_capacity_one_hundred() {
    let table: HashTable<&str> = HashTable::with_capacity(100);
}

#[test]
fn test_with_capacity_one_thousand_and_twenty_four() {
    let table: HashTable<&str> = HashTable::with_capacity(1024);
}

#[test]
fn test_with_capacity_two_thousand_and_forty_eight() {
    let table: HashTable<&str> = HashTable::with_capacity(2048);
}

#[test]
fn test_with_capacity_sixteen_bit_capacity() {
    let table: HashTable<&str> = HashTable::with_capacity(65536);
}

#[test]
fn test_with_capacity_max_usize() {
    let table: HashTable<&str> = HashTable::with_capacity(usize::MAX);
}

