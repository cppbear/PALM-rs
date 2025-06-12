// Answer 0

fn vb(index: usize) -> usize {
    index
}

#[derive(Debug)]
struct TestStruct(Vec<u8>);

const STATE_UNKNOWN: u8 = 0;
const STATE_DEAD: u8 = 1;

impl std::fmt::Debug for TestStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut fmtd = f.debug_map();
        for (b, si) in self.0.iter().enumerate() {
            match *si {
                STATE_UNKNOWN => {}
                STATE_DEAD => {
                    fmtd.entry(&vb(b as usize), &"DEAD");
                }
                si => {
                    fmtd.entry(&vb(b as usize), &si.to_string());
                }
            }
        }
        fmtd.finish()
    }
}

#[test]
fn test_empty_state() {
    let test_struct = TestStruct(vec![]);
    let result = format!("{:?}", test_struct);
    assert_eq!(result, "{}");
}

#[test]
fn test_single_dead_state() {
    let test_struct = TestStruct(vec![STATE_DEAD]);
    let result = format!("{:?}", test_struct);
    assert_eq!(result, "{0: \"DEAD\"}");
}

#[test]
fn test_single_unknown_state() {
    let test_struct = TestStruct(vec![STATE_UNKNOWN]);
    let result = format!("{:?}", test_struct);
    assert_eq!(result, "{}");
}

#[test]
fn test_mixed_states() {
    let test_struct = TestStruct(vec![STATE_DEAD, STATE_UNKNOWN, 2]);
    let result = format!("{:?}", test_struct);
    assert_eq!(result, "{0: \"DEAD\", 2: \"2\"}");
}

#[test]
fn test_all_states() {
    let test_struct = TestStruct(vec![STATE_DEAD, STATE_DEAD, STATE_UNKNOWN, 3]);
    let result = format!("{:?}", test_struct);
    assert_eq!(result, "{0: \"DEAD\", 1: \"DEAD\", 3: \"3\"}");
}

