fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Float))
    }