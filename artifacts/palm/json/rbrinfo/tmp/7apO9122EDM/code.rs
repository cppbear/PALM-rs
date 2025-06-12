fn unit_variant(self) -> Result<()> {
        de::Deserialize::deserialize(self.de)
    }