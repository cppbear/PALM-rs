// Answer 0

#[test]
fn test_swap_remove_finish_valid_case() {
    let mut indices = hash_table::HashTable::<usize>::default();
    let mut entries = vec![Bucket { hash: HashValue(1), key: 10, value: "Value1" }, 
                           Bucket { hash: HashValue(2), key: 20, value: "Value2" }];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_remove_finish(0);
}

#[test]
fn test_swap_remove_finish_middle_case() {
    let mut indices = hash_table::HashTable::<usize>::default();
    let mut entries = vec![Bucket { hash: HashValue(1), key: 10, value: "Value1" }, 
                           Bucket { hash: HashValue(2), key: 20, value: "Value2" }, 
                           Bucket { hash: HashValue(3), key: 30, value: "Value3" }];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_remove_finish(1);
}

#[test]
fn test_swap_remove_finish_last_case() {
    let mut indices = hash_table::HashTable::<usize>::default();
    let mut entries = vec![Bucket { hash: HashValue(1), key: 10, value: "Value1" }];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_remove_finish(0);
}

#[test]
#[should_panic]
fn test_swap_remove_finish_out_of_bounds_case() {
    let mut indices = hash_table::HashTable::<usize>::default();
    let mut entries = vec![Bucket { hash: HashValue(1), key: 10, value: "Value1" }];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_remove_finish(1);
}

