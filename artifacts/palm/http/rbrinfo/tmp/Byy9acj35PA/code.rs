pub fn clear(&mut self) {
        if let Some(ref mut map) = self.map {
            map.clear();
        }
    }