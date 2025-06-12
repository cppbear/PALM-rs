pub fn get_mut(&mut self) -> &mut V {
        let index = self.index();
        &mut self.entries[index].value
    }