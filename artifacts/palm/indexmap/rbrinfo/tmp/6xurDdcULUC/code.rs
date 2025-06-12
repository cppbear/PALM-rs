pub(super) fn into_muts(self) -> (&'a mut K, &'a mut V) {
        let index = self.index();
        self.entries[index].muts()
    }