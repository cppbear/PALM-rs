fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value, E>
        where
            V: Visitor<'de>,
        {
            // Covered by tests/test_enum_untagged.rs
            //      newtype_struct
            match *self.content {
                Content::Newtype(ref v) => {
                    visitor.visit_newtype_struct(ContentRefDeserializer::new(v))
                }
                // This case is to support data formats that encode newtype
                // structs and their underlying data the same, with no
                // indication whether a newtype wrapper was present. For example
                // JSON does this, while RON does not. In RON a newtype's name
                // is included in the serialized representation and it knows to
                // call `Visitor::visit_newtype_struct` from `deserialize_any`.
                // JSON's `deserialize_any` never calls `visit_newtype_struct`
                // but in this code we still must be able to deserialize the
                // resulting Content into newtypes.
                _ => visitor.visit_newtype_struct(self),
            }
        }