fn index_mut(&mut self, index: &Q) -> &mut Value {
        self.map.get_mut(index).expect("no entry found for key")
    }