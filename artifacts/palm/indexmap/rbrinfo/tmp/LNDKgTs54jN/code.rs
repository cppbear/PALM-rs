pub fn shift_insert(&mut self, index: usize, key: K, value: V) -> Option<V> {
        let len = self.len();
        match self.entry(key) {
            Entry::Occupied(mut entry) => {
                assert!(
                    index < len,
                    "index out of bounds: the len is {len} but the index is {index}"
                );

                let old = mem::replace(entry.get_mut(), value);
                entry.move_index(index);
                Some(old)
            }
            Entry::Vacant(entry) => {
                assert!(
                    index <= len,
                    "index out of bounds: the len is {len} but the index is {index}. Expected index <= len"
                );

                entry.shift_insert(index, value);
                None
            }
        }
    }