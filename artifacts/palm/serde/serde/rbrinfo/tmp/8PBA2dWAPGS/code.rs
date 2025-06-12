fn invalid_type(self, exp: &dyn Expected) -> E {
            de::Error::invalid_type(self.content.unexpected(), exp)
        }