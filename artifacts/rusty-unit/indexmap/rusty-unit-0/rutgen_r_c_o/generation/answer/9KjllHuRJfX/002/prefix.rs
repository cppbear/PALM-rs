// Answer 0

#[test]
fn test_swap_remove_full_valid_entry() {
    let mut index_map: IndexMapCore<usize, String> = IndexMapCore::new();
    let hash = HashValue(1);
    index_map.insert_full(hash, 1, "One".to_string());
    index_map.insert_full(HashValue(2), 2, "Two".to_string());
    
    let result = index_map.swap_remove_full(hash, &1);
}

#[test]
fn test_swap_remove_full_edge_case_first_entry() {
    let mut index_map: IndexMapCore<usize, String> = IndexMapCore::new();
    let hash = HashValue(3);
    index_map.insert_full(hash, 3, "Three".to_string());
    
    let result = index_map.swap_remove_full(hash, &3);
}

#[test]
fn test_swap_remove_full_edge_case_last_entry() {
    let mut index_map: IndexMapCore<usize, String> = IndexMapCore::new();
    let hash = HashValue(4);
    index_map.insert_full(hash, 4, "Four".to_string());
    index_map.insert_full(HashValue(5), 5, "Five".to_string());
    
    let result = index_map.swap_remove_full(hash, &4);
}

#[test]
fn test_swap_remove_full_multiple_entries() {
    let mut index_map: IndexMapCore<usize, String> = IndexMapCore::with_capacity(5);
    index_map.insert_full(HashValue(6), 6, "Six".to_string());
    index_map.insert_full(HashValue(7), 7, "Seven".to_string());
    index_map.insert_full(HashValue(8), 8, "Eight".to_string());
    
    let result = index_map.swap_remove_full(HashValue(7), &7);
}

#[test]
fn test_swap_remove_full_non_existent_key() {
    let mut index_map: IndexMapCore<usize, String> = IndexMapCore::new();
    index_map.insert_full(HashValue(9), 9, "Nine".to_string());
    
    let result = index_map.swap_remove_full(HashValue(10), &10);
}

