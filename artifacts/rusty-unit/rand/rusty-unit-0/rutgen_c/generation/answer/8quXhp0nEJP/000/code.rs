// Answer 0

#[test]
fn test_fmt_debug() {
    struct TestLcg128CmDxsm64 {
        state: u128,
        increment: u128,
    }
    
    impl fmt::Debug for TestLcg128CmDxsm64 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Lcg128CmDxsm64 {{}}")
        }
    }

    let rng = TestLcg128CmDxsm64 {
        state: 0,
        increment: 0,
    };
    
    let result = format!("{:?}", rng);
    assert_eq!(result, "Lcg128CmDxsm64 {{}}");
}

