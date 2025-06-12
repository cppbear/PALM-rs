fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut tag = None;
            let mut vec = Vec::<(Content, Content)>::with_capacity(size_hint::cautious::<(
                Content,
                Content,
            )>(map.size_hint()));
            while let Some(k) = tri!(map.next_key_seed(TagOrContentVisitor::new(self.tag_name))) {
                match k {
                    TagOrContent::Tag => {
                        if tag.is_some() {
                            return Err(de::Error::duplicate_field(self.tag_name));
                        }
                        tag = Some(tri!(map.next_value()));
                    }
                    TagOrContent::Content(k) => {
                        let v = tri!(map.next_value());
                        vec.push((k, v));
                    }
                }
            }
            match tag {
                None => Err(de::Error::missing_field(self.tag_name)),
                Some(tag) => Ok((tag, Content::Map(vec))),
            }
        }