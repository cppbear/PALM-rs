pub fn clear(&mut self) {
        self.entries.clear();
        self.extra_values.clear();
        self.danger = Danger::Green;

        for e in self.indices.iter_mut() {
            *e = Pos::none();
        }
    }