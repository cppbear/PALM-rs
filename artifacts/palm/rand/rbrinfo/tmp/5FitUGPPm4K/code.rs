fn random_bool(&mut self, p: f64) -> bool {
        match distr::Bernoulli::new(p) {
            Ok(d) => self.sample(d),
            Err(_) => panic!("p={:?} is outside range [0.0, 1.0]", p),
        }
    }