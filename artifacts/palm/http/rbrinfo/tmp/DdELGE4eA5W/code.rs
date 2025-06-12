fn value_iter(&self, idx: Option<usize>) -> ValueIter<'_, T> {
        use self::Cursor::*;

        if let Some(idx) = idx {
            let back = {
                let entry = &self.entries[idx];

                entry.links.map(|l| Values(l.tail)).unwrap_or(Head)
            };

            ValueIter {
                map: self,
                index: idx,
                front: Some(Head),
                back: Some(back),
            }
        } else {
            ValueIter {
                map: self,
                index: usize::MAX,
                front: None,
                back: None,
            }
        }
    }