// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    
    // Test with a single character string
    map.entry("a").or_insert_with(|| 42);
    
    // Test with a longer string
    map.entry("longer_key").or_insert_with(|| 100);
    
    // Test with another single character string
    map.entry("b").or_insert_with(|| -15);
    
    // Test with a key that gets another insertion
    map.entry("a").or_insert_with(|| 99);
    
    // Test with more keys to test capacity
    for i in 0..10000 {
        let key = format!("key_{}", i);
        map.entry(&key).or_insert_with(|| i as i32);
    }
} 

#[test]
#[should_panic]
fn test_or_insert_with_panic_on_insert_invalid() {
    let mut map: HashMap<&str, i32> = HashMap::new();

    // Trying to mutate while getting value for the same key
    *map.entry("poneyland").or_insert_with(|| {
        panic!("This should panic!");
    });
}

#[test]
fn test_or_insert_with_return_value() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    
    // Using or_insert_with to verify return value 
    let value1 = map.entry("test").or_insert_with(|| 10);
    *value1 += 5; // modify value
    
    let value2 = map.entry("test").or_insert_with(|| 20);
    
    // value2 should point to the same value as value1
    assert_eq!(*value1, *value2);
}

