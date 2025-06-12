pub fn raw_entry(&self) -> RawEntryBuilder<'_, K, V, S, A> {
        RawEntryBuilder { map: self }
    }