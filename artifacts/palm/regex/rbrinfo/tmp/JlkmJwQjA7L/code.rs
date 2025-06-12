fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut fmtd = f.debug_map();
        for (b, si) in self.0.iter().enumerate() {
            match *si {
                STATE_UNKNOWN => {}
                STATE_DEAD => {
                    fmtd.entry(&vb(b as usize), &"DEAD");
                }
                si => {
                    fmtd.entry(&vb(b as usize), &si.to_string());
                }
            }
        }
        fmtd.finish()
    }