fn insert_occupied_mult(&mut self, index: usize, value: T) -> ValueDrain<'_, T> {
        let old;
        let links;

        {
            let entry = &mut self.entries[index];

            old = mem::replace(&mut entry.value, value);
            links = entry.links.take();
        }

        let raw_links = self.raw_links();
        let extra_values = &mut self.extra_values;

        let next =
            links.map(|l| drain_all_extra_values(raw_links, extra_values, l.next).into_iter());

        ValueDrain {
            first: Some(old),
            next,
            lt: PhantomData,
        }
    }