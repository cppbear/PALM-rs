fn next(&mut self) -> Option<Self::Item> {
        use self::Cursor::*;

        if self.cursor.is_none() {
            if (self.entry + 1) >= self.map.entries.len() {
                return None;
            }

            self.entry += 1;
            self.cursor = Some(Cursor::Head);
        }

        let entry = &self.map.entries[self.entry];

        match self.cursor.unwrap() {
            Head => {
                self.cursor = entry.links.map(|l| Values(l.next));
                Some((&entry.key, &entry.value))
            }
            Values(idx) => {
                let extra = &self.map.extra_values[idx];

                match extra.next {
                    Link::Entry(_) => self.cursor = None,
                    Link::Extra(i) => self.cursor = Some(Values(i)),
                }

                Some((&entry.key, &extra.value))
            }
        }
    }