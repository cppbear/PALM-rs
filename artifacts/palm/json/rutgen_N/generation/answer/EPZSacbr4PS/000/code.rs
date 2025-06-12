// Answer 0

#[derive(Debug)]
struct Deserializer<R> {
    reader: R,
}

struct UnitVariantAccess<'a, R> {
    de: &'a mut Deserializer<R>,
}

impl<R> UnitVariantAccess<'_, R> {
    fn new(de: &mut Deserializer<R>) -> Self {
        UnitVariantAccess { de }
    }
}

#[test]
fn test_unit_variant_access_creation() {
    let mut deserializer = Deserializer { reader: "test_input" };
    let unit_variant_access = UnitVariantAccess::new(&mut deserializer);
    assert!(unit_variant_access.de.reader == "test_input");
}

