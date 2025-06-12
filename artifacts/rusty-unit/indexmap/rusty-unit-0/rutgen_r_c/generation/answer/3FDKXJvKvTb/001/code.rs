// Answer 0

#[test]
fn test_swap_indices_within_bounds() {
    struct TestK;
    struct TestV;

    let mut indices = vec![0, 1].into_boxed_slice();  
    let mut entries = vec![(HashValue::default(), TestK, TestV)].into_boxed_slice();
    
    let mut map = RefMut::new(&mut indices, &mut entries);
    let entry = IndexedEntry::new(&mut map, 0);
    
    // Swap within bounds; should work without panicking
    let mut entry_clone = entry.clone();
    entry_clone.swap_indices(1);
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds() {
    struct TestK;
    struct TestV;

    let mut indices = vec![0, 1].into_boxed_slice();  
    let mut entries = vec![(HashValue::default(), TestK, TestV)].into_boxed_slice();
    
    let mut map = RefMut::new(&mut indices, &mut entries);
    let entry = IndexedEntry::new(&mut map, 0);

    // Attempt to swap with an out-of-bounds index; should panic
    entry.swap_indices(2);
}

#[test]
fn test_swap_indices_same_index() {
    struct TestK;
    struct TestV;

    let mut indices = vec![0, 1].into_boxed_slice();  
    let mut entries = vec![(HashValue::default(), TestK, TestV)].into_boxed_slice();
    
    let mut map = RefMut::new(&mut indices, &mut entries);
    let entry = IndexedEntry::new(&mut map, 0);
    
    // Swap with itself; should work without panicking
    entry.swap_indices(0);
}

#[test]
#[should_panic]
fn test_swap_indices_self_panic_when_indices_equal() {
    struct TestK;
    struct TestV;

    let mut indices = vec![0, 1].into_boxed_slice();  
    let mut entries = vec![(HashValue::default(), TestK, TestV)].into_boxed_slice();
    
    let mut map = RefMut::new(&mut indices, &mut entries);
    let entry = IndexedEntry::new(&mut map, 1);
    
    // Test swapping with self to check for panic without boundaries error
    entry.swap_indices(1);
} 

#[test]
fn test_swap_indices_multiple_entries() {
    struct TestK;
    struct TestV;

    let mut indices = vec![0, 1, 2, 3].into_boxed_slice();  
    let mut entries = vec![
        (HashValue::default(), TestK, TestV),
        (HashValue::default(), TestK, TestV),
        (HashValue::default(), TestK, TestV),
        (HashValue::default(), TestK, TestV),
    ].into_boxed_slice();
    
    let mut map = RefMut::new(&mut indices, &mut entries);
    let entry = IndexedEntry::new(&mut map, 1);
    
    // Swap two different valid entries
    entry.swap_indices(2);
}

