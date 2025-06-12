pub fn remove_entry(self) -> (HeaderName, T) {
        if let Some(links) = self.map.entries[self.index].links {
            self.map.remove_all_extra_values(links.next);
        }

        let entry = self.map.remove_found(self.probe, self.index);

        (entry.key, entry.value)
    }