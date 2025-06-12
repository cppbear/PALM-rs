pub fn new(type_name: &'a str, variant_name: &'a str) -> Self {
            UntaggedUnitVisitor {
                type_name,
                variant_name,
            }
        }