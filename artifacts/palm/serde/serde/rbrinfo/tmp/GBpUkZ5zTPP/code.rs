pub fn new(type_name: &'a str, variant_name: &'a str) -> Self {
            InternallyTaggedUnitVisitor {
                type_name,
                variant_name,
            }
        }