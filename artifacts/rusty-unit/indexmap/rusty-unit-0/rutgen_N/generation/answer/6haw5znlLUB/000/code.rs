// Answer 0

#[derive(Default)]
struct Indices {
    // fields for Indices
}

#[derive(Default)]
struct Entries<K, V> {
    // fields for Entries
}

struct MyStruct<'a, K, V> {
    indices: &'a mut Indices,
    entries: &'a mut Entries<K, V>,
}

impl<'a, K, V> MyStruct<'a, K, V> {
    fn new(indices: &'a mut Indices, entries: &'a mut Entries<K, V>) -> Self {
        Self { indices, entries }
    }
}

#[test]
fn test_my_struct_new() {
    let mut indices = Indices::default();
    let mut entries = Entries::<i32, String>::default();
    let my_struct = MyStruct::new(&mut indices, &mut entries);
    assert!(!std::ptr::eq(my_struct.indices, &mut indices));
    assert!(!std::ptr::eq(my_struct.entries, &mut entries));
}

#[test]
fn test_my_struct_new_with_different_types() {
    let mut indices = Indices::default();
    let mut entries = Entries::<String, usize>::default();
    let my_struct = MyStruct::new(&mut indices, &mut entries);
    assert!(!std::ptr::eq(my_struct.indices, &mut indices));
    assert!(!std::ptr::eq(my_struct.entries, &mut entries));
}

