fn clone_from(&mut self, source: &Self) {
        self.map.clone_from(&source.map);
    }