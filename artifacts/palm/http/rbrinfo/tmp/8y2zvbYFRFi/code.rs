fn extend<I: IntoIterator<Item = (Option<HeaderName>, T)>>(&mut self, iter: I) {
        let mut iter = iter.into_iter();

        // The structure of this is a bit weird, but it is mostly to make the
        // borrow checker happy.
        let (mut key, mut val) = match iter.next() {
            Some((Some(key), val)) => (key, val),
            Some((None, _)) => panic!("expected a header name, but got None"),
            None => return,
        };

        'outer: loop {
            let mut entry = match self.try_entry2(key).expect("size overflows MAX_SIZE") {
                Entry::Occupied(mut e) => {
                    // Replace all previous values while maintaining a handle to
                    // the entry.
                    e.insert(val);
                    e
                }
                Entry::Vacant(e) => e.insert_entry(val),
            };

            // As long as `HeaderName` is none, keep inserting the value into
            // the current entry
            loop {
                match iter.next() {
                    Some((Some(k), v)) => {
                        key = k;
                        val = v;
                        continue 'outer;
                    }
                    Some((None, v)) => {
                        entry.append(v);
                    }
                    None => {
                        return;
                    }
                }
            }
        }
    }