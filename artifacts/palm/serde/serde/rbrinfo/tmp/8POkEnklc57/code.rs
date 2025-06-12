fn unit_variant(self) -> Result<(), E> {
            match self.value {
                Some(value) => de::Deserialize::deserialize(ContentDeserializer::new(value)),
                None => Ok(()),
            }
        }