fn remove_extra_value(&mut self, idx: usize) -> ExtraValue<T> {
        let raw_links = self.raw_links();
        remove_extra_value(raw_links, &mut self.extra_values, idx)
    }