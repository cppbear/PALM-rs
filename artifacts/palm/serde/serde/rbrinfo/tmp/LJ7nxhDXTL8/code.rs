fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(self.bad_type(Unsupported::Sequence))
    }