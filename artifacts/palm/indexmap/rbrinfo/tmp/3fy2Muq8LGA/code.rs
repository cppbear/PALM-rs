pub fn into_mut(self) -> &'a mut V {
        let index = self.index();
        &mut self.entries[index].value
    }