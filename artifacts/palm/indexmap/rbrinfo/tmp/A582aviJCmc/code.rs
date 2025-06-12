pub fn insert_before(&mut self, mut index: usize, key: K, value: V) -> (usize, Option<V>) {
        let len = self.len();

        assert!(
            index <= len,
            "index out of bounds: the len is {len} but the index is {index}. Expected index <= len"
        );

        match self.entry(key) {
            Entry::Occupied(mut entry) => {
                if index > entry.index() {
                    // Some entries will shift down when this one moves up,
                    // so "insert before index" becomes "move to index - 1",
                    // keeping the entry at the original index unmoved.
                    index -= 1;
                }
                let old = mem::replace(entry.get_mut(), value);
                entry.move_index(index);
                (index, Some(old))
            }
            Entry::Vacant(entry) => {
                entry.shift_insert(index, value);
                (index, None)
            }
        }
    }