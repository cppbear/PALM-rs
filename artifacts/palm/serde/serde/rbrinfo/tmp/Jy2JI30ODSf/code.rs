fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut vec =
                Vec::<Content>::with_capacity(size_hint::cautious::<Content>(visitor.size_hint()));
            while let Some(e) = tri!(visitor.next_element()) {
                vec.push(e);
            }
            Ok(Content::Seq(vec))
        }