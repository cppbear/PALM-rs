fn get_index_mut2(&mut self, index: usize) -> Option<&mut T> {
        match self.map.get_index_mut2(index) {
            Some((value, ())) => Some(value),
            None => None,
        }
    }