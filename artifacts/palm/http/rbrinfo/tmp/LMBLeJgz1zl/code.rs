fn insert_occupied(&mut self, index: usize, value: T) -> T {
        if let Some(links) = self.entries[index].links {
            self.remove_all_extra_values(links.next);
        }

        let entry = &mut self.entries[index];
        mem::replace(&mut entry.value, value)
    }