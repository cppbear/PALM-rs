fn next_unsafe(&mut self) -> Option<(&'a HeaderName, *mut T)> {
        use self::Cursor::*;

        if self.cursor.is_none() {
            if (self.entry + 1) >= unsafe { &*self.map }.entries.len() {
                return None;
            }

            self.entry += 1;
            self.cursor = Some(Cursor::Head);
        }

        let entry = unsafe { &mut (*self.map).entries[self.entry] };

        match self.cursor.unwrap() {
            Head => {
                self.cursor = entry.links.map(|l| Values(l.next));
                Some((&entry.key, &mut entry.value as *mut _))
            }
            Values(idx) => {
                let extra = unsafe { &mut (*self.map).extra_values[idx] };

                match extra.next {
                    Link::Entry(_) => self.cursor = None,
                    Link::Extra(i) => self.cursor = Some(Values(i)),
                }

                Some((&entry.key, &mut extra.value as *mut _))
            }
        }
    }