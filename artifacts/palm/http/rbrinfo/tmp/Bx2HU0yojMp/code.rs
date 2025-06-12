fn raw_links(&mut self) -> RawLinks<T> {
        RawLinks(&mut self.entries[..] as *mut _)
    }