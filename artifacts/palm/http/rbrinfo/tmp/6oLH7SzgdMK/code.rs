fn next(&mut self) -> Option<Self::Item> {
        self.next_unsafe()
            .map(|(key, ptr)| (key, unsafe { &mut *ptr }))
    }