pub fn remove_entry_mult(self) -> (HeaderName, ValueDrain<'a, T>) {
        let raw_links = self.map.raw_links();
        let extra_values = &mut self.map.extra_values;

        let next = self.map.entries[self.index]
            .links
            .map(|l| drain_all_extra_values(raw_links, extra_values, l.next).into_iter());

        let entry = self.map.remove_found(self.probe, self.index);

        let drain = ValueDrain {
            first: Some(entry.value),
            next,
            lt: PhantomData,
        };
        (entry.key, drain)
    }