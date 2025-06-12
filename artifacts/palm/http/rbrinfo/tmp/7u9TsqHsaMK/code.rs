pub fn drain(&mut self) -> Drain<'_, T> {
        for i in self.indices.iter_mut() {
            *i = Pos::none();
        }

        // Memory safety
        //
        // When the Drain is first created, it shortens the length of
        // the source vector to make sure no uninitialized or moved-from
        // elements are accessible at all if the Drain's destructor never
        // gets to run.

        let entries = &mut self.entries[..] as *mut _;
        let extra_values = &mut self.extra_values as *mut _;
        let len = self.entries.len();
        unsafe {
            self.entries.set_len(0);
        }

        Drain {
            idx: 0,
            len,
            entries,
            extra_values,
            next: None,
            lt: PhantomData,
        }
    }