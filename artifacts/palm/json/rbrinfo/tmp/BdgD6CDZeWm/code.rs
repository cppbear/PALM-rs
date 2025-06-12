pub fn as_f64(&self) -> Option<f64> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n as f64),
            N::NegInt(n) => Some(n as f64),
            N::Float(n) => Some(n),
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse::<f64>().ok().filter(|float| float.is_finite())
    }