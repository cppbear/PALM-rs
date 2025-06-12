fn next_back(&mut self) -> Option<Self::Item> {
        use self::Cursor::*;

        match self.back {
            Some(Head) => {
                self.front = None;
                self.back = None;
                Some(&self.map.entries[self.index].value)
            }
            Some(Values(idx)) => {
                let extra = &self.map.extra_values[idx];

                if self.front == self.back {
                    self.front = None;
                    self.back = None;
                } else {
                    match extra.prev {
                        Link::Entry(_) => self.back = Some(Head),
                        Link::Extra(idx) => self.back = Some(Values(idx)),
                    }
                }

                Some(&extra.value)
            }
            None => None,
        }
    }