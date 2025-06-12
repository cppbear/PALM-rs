// Answer 0

#[derive(Debug)]
struct TestEnumAccess {
    variant: Option<()>,
}

impl<'de> EnumAccess<'de> for TestEnumAccess {
    type Error = ();
    type Variants = TestVariants;

    fn variant<V>(self) -> Result<(V, Self), Self::Error>
    where
        V: VariantAccess<'de>,
    {
        if self.variant.is_some() {
            Ok(((), self))
        } else {
            Err(())
        }
    }
}

struct TestVariants;

impl Iterator for TestVariants {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        Some(())
    }
}

#[derive(Debug)]
struct IgnoredAny;

struct Visitor;

impl Visitor {
    fn visit_enum<A>(self, data: A) -> Result<IgnoredAny, A::Error>
    where
        A: EnumAccess<'de>,
    {
        data.variant::<IgnoredAny>().map(|(_, _)| IgnoredAny)
    }
}

#[test]
fn test_visit_enum_success() {
    let test_access = TestEnumAccess { variant: Some(()) };
    let visitor = Visitor;
    let result = visitor.visit_enum(test_access);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_visit_enum_failure() {
    let test_access = TestEnumAccess { variant: None };
    let visitor = Visitor;
    let _result = visitor.visit_enum(test_access).unwrap();
}

