// Answer 0

#[test]
fn test_new_valid_inputs() {
    struct DummyIndices;
    struct DummyEntries<K, V> {
        _marker: std::marker::PhantomData<(K, V)>
    }
    struct ResultStruct<'a, K, V> {
        indices: &'a mut DummyIndices,
        entries: &'a mut DummyEntries<K, V>
    }

    let mut indices = DummyIndices;
    let mut entries = DummyEntries { _marker: std::marker::PhantomData };
    
    let result = ResultStruct::new(&mut indices, &mut entries);
    assert!(std::ptr::eq(result.indices, &mut indices));
    assert!(std::ptr::eq(result.entries, &mut entries));
}

#[test]
#[should_panic]
fn test_new_panic_on_invalid_inputs() {
    struct DummyIndices;
    struct DummyEntries<K, V> {
        _marker: std::marker::PhantomData<(K, V)>
    }
    struct ResultStruct<'a, K, V> {
        indices: &'a mut DummyIndices,
        entries: &'a mut DummyEntries<K, V>
    }

    let indices: *mut DummyIndices = std::ptr::null_mut();
    let entries: *mut DummyEntries<i32, i32> = std::ptr::null_mut();
    
    // This line should panic since the references are null.
    ResultStruct::new(&mut *(indices), &mut *(entries));
}

