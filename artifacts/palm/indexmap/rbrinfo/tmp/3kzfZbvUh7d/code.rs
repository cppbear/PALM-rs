fn raw_entry_mut_v1(&mut self) -> RawEntryBuilderMut<'_, K, V, S> {
        RawEntryBuilderMut { map: self }
    }