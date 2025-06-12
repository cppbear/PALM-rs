fn next(&mut self) -> Option<Self::Item> {
        use self::Cursor::*;

        match self.front {
            Some(Head) => {
                let entry = &self.map.entries[self.index];

                if self.back == Some(Head) {
                    self.front = None;
                    self.back = None;
                } else {
                    // Update the iterator state
                    match entry.links {
                        Some(links) => {
                            self.front = Some(Values(links.next));
                        }
                        None => unreachable!(),
                    }
                }

                Some(&entry.value)
            }
            Some(Values(idx)) => {
                let extra = &self.map.extra_values[idx];

                if self.front == self.back {
                    self.front = None;
                    self.back = None;
                } else {
                    match extra.next {
                        Link::Entry(_) => self.front = None,
                        Link::Extra(i) => self.front = Some(Values(i)),
                    }
                }

                Some(&extra.value)
            }
            None => None,
        }
    }