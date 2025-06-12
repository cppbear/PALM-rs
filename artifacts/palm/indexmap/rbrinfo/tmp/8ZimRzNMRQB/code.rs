fn clone_from(&mut self, other: &Self) {
        self.map.clone_from(&other.map);
    }