pub fn append<S2>(&mut self, other: &mut IndexMap<K, V, S2>) {
        self.extend(other.drain(..));
    }