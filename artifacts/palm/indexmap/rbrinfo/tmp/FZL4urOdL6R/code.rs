fn from(other: IndexedEntry<'a, K, V>) -> Self {
        let IndexedEntry {
            map: RefMut { indices, entries },
            index,
        } = other;
        let hash = entries[index].hash;
        Self {
            entries,
            index: indices
                .find_entry(hash.get(), move |&i| i == index)
                .expect("index not found"),
        }
    }