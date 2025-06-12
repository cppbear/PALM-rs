// Answer 0

#[derive(Debug)]
struct TestStruct {
    kind: TestKind,
}

#[derive(Debug)]
enum TestKind {
    TypeA,
    TypeB,
}

impl TestKind {
    fn description(&self) -> &str {
        match self {
            TestKind::TypeA => "Description for Type A",
            TestKind::TypeB => "Description for Type B",
        }
    }
}

impl TestStruct {
    fn description(&self) -> &str {
        self.kind.description()
    }
}

#[test]
fn test_description_type_a() {
    let test_struct = TestStruct {
        kind: TestKind::TypeA,
    };
    assert_eq!(test_struct.description(), "Description for Type A");
}

#[test]
fn test_description_type_b() {
    let test_struct = TestStruct {
        kind: TestKind::TypeB,
    };
    assert_eq!(test_struct.description(), "Description for Type B");
}

