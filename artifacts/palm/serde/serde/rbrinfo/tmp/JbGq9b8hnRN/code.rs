fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, E>
        where
            V: Visitor<'de>,
        {
            // Covered by tests/test_enum_untagged.rs
            //      with_optional_field::*
            match *self.content {
                Content::None => visitor.visit_none(),
                Content::Some(ref v) => visitor.visit_some(ContentRefDeserializer::new(v)),
                Content::Unit => visitor.visit_unit(),
                // This case is to support data formats which do not encode an
                // indication whether a value is optional. An example of such a
                // format is JSON, and a counterexample is RON. When requesting
                // `deserialize_any` in JSON, the data format never performs
                // `Visitor::visit_some` but we still must be able to
                // deserialize the resulting Content into data structures with
                // optional fields.
                _ => visitor.visit_some(self),
            }
        }