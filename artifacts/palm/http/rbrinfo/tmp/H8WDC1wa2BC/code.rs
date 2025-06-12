fn value_iter_mut(&mut self, idx: usize) -> ValueIterMut<'_, T> {
        use self::Cursor::*;

        let back = {
            let entry = &self.entries[idx];

            entry.links.map(|l| Values(l.tail)).unwrap_or(Head)
        };

        ValueIterMut {
            map: self as *mut _,
            index: idx,
            front: Some(Head),
            back: Some(back),
            lt: PhantomData,
        }
    }