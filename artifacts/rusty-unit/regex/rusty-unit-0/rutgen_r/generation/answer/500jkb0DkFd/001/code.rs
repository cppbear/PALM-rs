// Answer 0

#[derive(Debug)]
struct KindA;

impl KindA {
    fn description(&self) -> &str {
        "KindA description"
    }
}

#[derive(Debug)]
struct KindB;

impl KindB {
    fn description(&self) -> &str {
        "KindB description"
    }
}

struct TestStruct {
    kind: Box<dyn KindTrait>,
}

trait KindTrait {
    fn description(&self) -> &str;
}

impl KindTrait for KindA {
    fn description(&self) -> &str {
        "KindA description"
    }
}

impl KindTrait for KindB {
    fn description(&self) -> &str {
        "KindB description"
    }
}

#[test]
fn test_description_kind_a() {
    let instance = TestStruct { kind: Box::new(KindA) };
    assert_eq!(instance.kind.description(), "KindA description");
}

#[test]
fn test_description_kind_b() {
    let instance = TestStruct { kind: Box::new(KindB) };
    assert_eq!(instance.kind.description(), "KindB description");
}

