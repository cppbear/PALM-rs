fn drop(&mut self) {
        // Finish draining unconsumed items. We don't strictly *have* to do this
        // manually, since we already split it into separate memory, but it will
        // match the drop order of `vec::Splice` items this way.
        let _ = self.drain.nth(usize::MAX);

        // Now insert all the new items. If a key matches an existing entry, it
        // keeps the original position and only replaces the value, like `insert`.
        while let Some((key, value)) = self.replace_with.next() {
            // Since the tail is disjoint, we can try to update it first,
            // or else insert (update or append) the primary map.
            let hash = self.map.hash(&key);
            if let Some(i) = self.tail.get_index_of(hash, &key) {
                self.tail.as_entries_mut()[i].value = value;
            } else {
                self.map.core.insert_full(hash, key, value);
            }
        }

        // Finally, re-append the tail
        self.map.core.append_unchecked(&mut self.tail);
    }