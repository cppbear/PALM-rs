// Answer 0

#[test]
fn test_extract_if_evens() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    // Create a hash table and initialize it with some values
    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    for x in 0..8 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    
    // Extract even numbers
    let drained: Vec<i32> = table.extract_if(|&mut v| v % 2 == 0).collect();

    // Collect remaining odd numbers
    let mut evens = drained.into_iter().collect::<Vec<_>>();
    let mut odds = table.into_iter().collect::<Vec<_>>();
    evens.sort();
    odds.sort();

    // Assert the expected results
    assert_eq!(evens, vec![0, 2, 4, 6]);
    assert_eq!(odds, vec![1, 3, 5, 7]);
}

#[test]
fn test_extract_if_no_matches() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    // Create a hash table with values
    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);
    
    for x in 1..5 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    
    // Extract based on a predicate that matches no values
    let drained: Vec<i32> = table.extract_if(|&mut v| v > 10).collect();
    
    // Collect remaining values
    let remaining: Vec<i32> = table.into_iter().collect();
    
    // Assert no values matched
    assert_eq!(drained, vec![]);
    assert_eq!(remaining, vec![1, 2, 3, 4]);
}

#[test]
#[should_panic]
fn test_extract_if_panic_on_empty_table() {
    use hashbrown::{HashTable, DefaultHashBuilder};

    // Create an empty hash table
    let mut table: HashTable<i32, i32, DefaultHashBuilder> = HashTable::new();
    
    // Attempt to extract from an empty table
    let _drained: Vec<i32> = table.extract_if(|&mut v| v % 2 == 0).collect();
}

