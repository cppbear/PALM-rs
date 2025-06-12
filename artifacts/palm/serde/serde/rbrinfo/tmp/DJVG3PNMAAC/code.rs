fn serialize_unit_struct(self, name: &'static str) -> Result<Content, E> {
            Ok(Content::UnitStruct(name))
        }