// Answer 0

#[derive(Debug)]
struct TestStruct {
    entries: Vec<i32>,
    indices: Vec<usize>,
}

impl TestStruct {
    fn new(entries: Vec<i32>, indices: Vec<usize>) -> Self {
        Self { entries, indices }
    }

    fn erase_indices(&mut self, start: usize, end: usize) {
        let (init, shifted_entries) = self.entries.split_at(end);
        let (start_entries, erased_entries) = init.split_at(start);

        let erased = erased_entries.len();
        let shifted = shifted_entries.len();
        let half_capacity = self.indices.capacity() / 2;

        if erased == 0 {
        } else if start + shifted < half_capacity && start < erased {
            self.indices.clear();
            self.indices.extend_from_slice(start_entries);
            self.indices.extend_from_slice(shifted_entries);
        } else if erased + shifted < half_capacity {
            for (i, &entry) in (start..).zip(erased_entries.iter()) {
                self.indices.retain(|&x| x != entry as usize);
            }
            for ((new, old), &entry) in (start..).zip(end..).zip(shifted_entries.iter()) {
                self.indices.iter_mut().for_each(|x| {
                    if *x == old as usize {
                        *x = new;
                    }
                });
            }
        } else {
            let offset = end - start;
            self.indices.retain(move |i| {
                if *i >= end {
                    *i -= offset;
                    true
                } else {
                    *i < start
                }
            });
        }
        
        debug_assert_eq!(self.indices.len(), start + shifted);
    }
}

#[test]
fn test_erase_with_erased_zero() {
    let mut instance = TestStruct::new(vec![1, 2, 3, 4, 5, 6], vec![0, 1, 2, 3]);
    instance.erase_indices(2, 2);
    assert_eq!(instance.indices, vec![0, 1, 2, 3]);
}

#[test]
fn test_erase_start_plus_shifted_less_half_capacity() {
    let mut instance = TestStruct::new(vec![1, 2, 3, 4, 5], vec![0, 1, 2, 3]);
    instance.erase_indices(0, 2);
    assert_eq!(instance.indices, vec![2, 3]);
}

#[test]
fn test_erase_erased_plus_shifted_less_half_capacity() {
    let mut instance = TestStruct::new(vec![1, 2, 3, 4, 5], vec![1, 2, 3]);
    instance.erase_indices(0, 3);
    assert_eq!(instance.indices, vec![2, 3]);
}

#[test]
fn test_erase_entries() {
    let mut instance = TestStruct::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], vec![0, 1, 2, 3, 4]);
    instance.erase_indices(4, 7);
    assert_eq!(instance.indices, vec![0, 1, 2, 3]);
}

#[test]
fn test_erase_non_matching() {
    let mut instance = TestStruct::new(vec![1, 2, 3, 4], vec![0, 1, 2]);
    instance.erase_indices(1, 3);
    assert!(instance.indices.contains(&0));
    assert!(!instance.indices.contains(&1));
    assert!(!instance.indices.contains(&2));
}

