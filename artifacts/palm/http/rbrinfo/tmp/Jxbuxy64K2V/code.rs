fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next {
            self.next = match self.extra_values[next].next {
                Link::Entry(_) => None,
                Link::Extra(v) => Some(v),
            };

            let value = unsafe { ptr::read(&self.extra_values[next].value) };

            return Some((None, value));
        }

        if let Some(bucket) = self.entries.next() {
            self.next = bucket.links.map(|l| l.next);
            let name = Some(bucket.key);
            let value = bucket.value;

            return Some((name, value));
        }

        None
    }