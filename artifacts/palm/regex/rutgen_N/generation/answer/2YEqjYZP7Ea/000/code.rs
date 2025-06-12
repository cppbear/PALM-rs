// Answer 0

#[derive(Debug)]
struct ExecNoSyncStr;

struct MyStruct;

impl MyStruct {
    fn searcher(&self) -> ExecNoSyncStr {
        ExecNoSyncStr
    }

    fn searcher_str(&self) -> ExecNoSyncStr {
        ExecNoSyncStr(self.searcher())
    }
}

#[test]
fn test_searcher_str() {
    let instance = MyStruct;
    let result = instance.searcher_str();
    assert!(std::mem::discriminant(&result) == std::mem::discriminant(&ExecNoSyncStr));
}

