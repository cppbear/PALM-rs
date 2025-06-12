// Answer 0

#[test]
fn test_deref_mut_valid() {
    struct MyStruct {
        value: i32,
    }
    
    let mut my_value = MyStruct { value: 10 };
    let mut scope_guard = ScopeGuard {
        dropfn: |val: &mut MyStruct| { val.value += 1; },
        value: my_value,
    };
    
    let mutable_ref: &mut MyStruct = scope_guard.deref_mut();
}

#[test]
fn test_deref_mut_another_valid() {
    struct AnotherStruct {
        name: String,
    }
    
    let mut another_value = AnotherStruct { name: String::from("test") };
    let mut scope_guard = ScopeGuard {
        dropfn: |val: &mut AnotherStruct| { val.name.push_str(" modified"); },
        value: another_value,
    };
    
    let mutable_ref: &mut AnotherStruct = scope_guard.deref_mut();
}

#[test]
fn test_deref_mut_edge_case() {
    struct EdgeCaseStruct {
        data: Vec<i32>,
    }
    
    let mut edge_case_value = EdgeCaseStruct { data: vec![1, 2, 3] };
    let mut scope_guard = ScopeGuard {
        dropfn: |val: &mut EdgeCaseStruct| { val.data.push(4); },
        value: edge_case_value,
    };
    
    let mutable_ref: &mut EdgeCaseStruct = scope_guard.deref_mut();
}

