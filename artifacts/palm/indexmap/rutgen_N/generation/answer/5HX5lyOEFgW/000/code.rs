// Answer 0

#[test]
fn test_into_ref_mut() {
    use std::cell::RefMut;

    struct MockIndex {
        table: Vec<(i32, i32)>,
    }

    struct MockEntry<'a> {
        index: MockIndex,
        entries: &'a mut [(i32, i32)],
    }

    impl<'a> MockEntry<'a> {
        fn into_ref_mut(self) -> RefMut<'a, i32, i32> {
            RefMut::new(self.index.table.as_slice(), self.entries)
        }
    }

    let mut entries = [(1, 10), (2, 20)];
    let index = MockIndex { table: vec![(1, 10), (2, 20)] };
    let entry = MockEntry { index, entries: &mut entries };

    let ref_mut = entry.into_ref_mut();

    assert_eq!(ref_mut.len(), 2);
    assert_eq!(ref_mut[0], (1, 10));
    assert_eq!(ref_mut[1], (2, 20));
}

