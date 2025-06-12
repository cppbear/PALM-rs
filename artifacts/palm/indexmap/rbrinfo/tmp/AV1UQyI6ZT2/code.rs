fn raw_entry_v1(&self) -> RawEntryBuilder<'_, K, V, S> {
        RawEntryBuilder { map: self }
    }