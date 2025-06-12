pub fn p(&self) -> f64 {
        if self.p_int == ALWAYS_TRUE {
            1.0
        } else {
            (self.p_int as f64) / SCALE
        }
    }