// Answer 0

#[test]
fn test_index_empty() {
    let indices = &mut Vec::<usize>::new();
    let entries = &mut Vec::<u32>::new(); 
    let mut ref_mut = RefMut {
        indices,
        entries,
    };
    let raw_entry = RawVacantEntryMut {
        map: ref_mut,
        hash_builder: &(),
    };
    assert_eq!(raw_entry.index(), 0);
}

#[test]
fn test_index_non_empty() {
    let indices = &mut vec![0, 1, 2];
    let entries = &mut Vec::<u32>::new(); 
    let mut ref_mut = RefMut {
        indices,
        entries,
    };
    let raw_entry = RawVacantEntryMut {
        map: ref_mut,
        hash_builder: &(),
    };
    assert_eq!(raw_entry.index(), 3);
}

#[test]
fn test_index_large_vector() {
    let indices = &mut (0..1000).collect::<Vec<usize>>();
    let entries = &mut Vec::<u32>::new(); 
    let mut ref_mut = RefMut {
        indices,
        entries,
    };
    let raw_entry = RawVacantEntryMut {
        map: ref_mut,
        hash_builder: &(),
    };
    assert_eq!(raw_entry.index(), 1000);
}

