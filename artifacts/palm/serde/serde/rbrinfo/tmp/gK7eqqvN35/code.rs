fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Float))
    }