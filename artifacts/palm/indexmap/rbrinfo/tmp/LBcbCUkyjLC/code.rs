pub(crate) fn append_unchecked(&mut self, other: &mut Self) {
        self.reserve(other.len());
        insert_bulk_no_grow(&mut self.indices, &other.entries);
        self.entries.append(&mut other.entries);
        other.indices.clear();
    }