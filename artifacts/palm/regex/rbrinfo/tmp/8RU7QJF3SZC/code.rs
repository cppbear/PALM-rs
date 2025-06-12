fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut fmtd = f.debug_map();
        for si in 0..self.num_states() {
            let s = si * self.num_byte_classes;
            let e = s + self.num_byte_classes;
            fmtd.entry(&si.to_string(), &TransitionsRow(&self.table[s..e]));
        }
        fmtd.finish()
    }