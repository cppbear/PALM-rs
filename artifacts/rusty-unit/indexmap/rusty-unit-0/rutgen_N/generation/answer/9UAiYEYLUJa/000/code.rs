// Answer 0

#[test]
fn test_swap_indices_valid() {
    struct Entry {
        index: usize,
    }

    impl Entry {
        fn index(&self) -> usize {
            self.index
        }

        fn into_ref_mut(self) -> EntryRefMut {
            EntryRefMut { entry: self }
        }
    }

    struct EntryRefMut {
        entry: Entry,
    }

    impl EntryRefMut {
        fn swap_indices(&mut self, index1: usize, index2: usize) {
            // Mocking swap logic, for testing purpose we are just checking the indices
            assert!(index1 < 2 && index2 < 2, "Index out of bounds");
        }
    }

    let entry1 = Entry { index: 0 };
    let entry2 = Entry { index: 1 };
    
    let mut ref_mut_entry1 = entry1.into_ref_mut();
    ref_mut_entry1.swap_indices(ref_mut_entry1.entry.index(), entry2.index());
}

#[should_panic(expected = "Index out of bounds")]
#[test]
fn test_swap_indices_out_of_bounds() {
    struct Entry {
        index: usize,
    }

    impl Entry {
        fn index(&self) -> usize {
            self.index
        }

        fn into_ref_mut(self) -> EntryRefMut {
            EntryRefMut { entry: self }
        }
    }

    struct EntryRefMut {
        entry: Entry,
    }

    impl EntryRefMut {
        fn swap_indices(&mut self, index1: usize, index2: usize) {
            // Mocking swap logic, for testing purpose we are just checking the indices
            assert!(index1 < 2 && index2 < 2, "Index out of bounds");
        }
    }

    let entry1 = Entry { index: 0 };
    let entry2 = Entry { index: 2 }; // Out of bounds index
    
    let mut ref_mut_entry1 = entry1.into_ref_mut();
    ref_mut_entry1.swap_indices(ref_mut_entry1.entry.index(), entry2.index());
}

