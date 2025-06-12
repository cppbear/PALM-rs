// Answer 0

fn shift_insert_within_bounds() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let hasher = SomeHasher; // Assuming SomeHasher implements BuildHasher
    let mut map = RefMut { indices: &mut indices, entries: &mut entries };
    let mut raw_entry = RawVacantEntryMut { map: &mut map, hash_builder: &hasher };

    let index = 0; // within bounds
    let key = "key1"; // valid K
    let value = "value1"; // valid V
    raw_entry.shift_insert(index, key, value);
}

fn shift_insert_empty_map() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let hasher = SomeHasher;
    let mut map = RefMut { indices: &mut indices, entries: &mut entries };
    let mut raw_entry = RawVacantEntryMut { map: &mut map, hash_builder: &hasher };

    let index = 0; // within bounds, map is empty
    let key = "key1";
    let value = "value1";
    raw_entry.shift_insert(index, key, value);
}

#[should_panic]
fn shift_insert_out_of_bounds() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let hasher = SomeHasher;
    let mut map = RefMut { indices: &mut indices, entries: &mut entries };
    let mut raw_entry = RawVacantEntryMut { map: &mut map, hash_builder: &hasher };

    let index = 1; // out of bounds
    let key = "key1";
    let value = "value1";
    raw_entry.shift_insert(index, key, value);
}

fn shift_insert_middle_of_map() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let hasher = SomeHasher;
    let mut map = RefMut { indices: &mut indices, entries: &mut entries };
    let mut raw_entry = RawVacantEntryMut { map: &mut map, hash_builder: &hasher };

    let index = 1; // valid position in a non-empty map
    let key1 = "key1";
    let value1 = "value1";
    raw_entry.shift_insert(0, key1, value1); // insert 1st item
    let key2 = "key2"; 
    let value2 = "value2";
    raw_entry.shift_insert(index, key2, value2); // insert in the middle
}

fn shift_insert_multiple() {
    let mut indices = Indices::new();
    let mut entries = Entries::new();
    let hasher = SomeHasher;
    let mut map = RefMut { indices: &mut indices, entries: &mut entries };
    let mut raw_entry = RawVacantEntryMut { map: &mut map, hash_builder: &hasher };

    let index1 = 0; 
    let key1 = "key1"; 
    let value1 = "value1"; 
    raw_entry.shift_insert(index1, key1, value1); 

    let index2 = 1; 
    let key2 = "key2"; 
    let value2 = "value2"; 
    raw_entry.shift_insert(index2, key2, value2); 
    
    let index3 = 2; 
    let key3 = "key3"; 
    let value3 = "value3"; 
    raw_entry.shift_insert(index3, key3, value3); 
}

