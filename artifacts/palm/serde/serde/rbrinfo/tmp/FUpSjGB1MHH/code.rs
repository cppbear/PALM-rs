fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let tag = match tri!(seq.next_element()) {
                Some(tag) => tag,
                None => {
                    return Err(de::Error::missing_field(self.tag_name));
                }
            };
            let rest = de::value::SeqAccessDeserializer::new(seq);
            Ok((tag, tri!(Content::deserialize(rest))))
        }