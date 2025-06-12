fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R) {
                for elt in self.iter_mut() {
                    *elt = rng.random();
                }
            }