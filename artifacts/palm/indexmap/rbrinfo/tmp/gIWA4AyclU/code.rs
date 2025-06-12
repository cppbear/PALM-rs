pub fn append<S2>(&mut self, other: &mut IndexSet<T, S2>) {
        self.map.append(&mut other.map);
    }