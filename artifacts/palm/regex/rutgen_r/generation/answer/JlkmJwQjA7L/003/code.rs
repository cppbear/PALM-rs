// Answer 0

fn vb(idx: usize) -> String {
    format!("State {}", idx)
}

const STATE_UNKNOWN: usize = 0;
const STATE_DEAD: usize = 1;

struct Dfa(Vec<usize>);

impl Dfa {
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
fn test_fmt_empty() {
    let dfa = Dfa(vec![]);
    let result = format!("{:?}", dfa);
    assert_eq!(result, "{}");
}

#[test]
fn test_fmt_all_unknown() {
    let dfa = Dfa(vec![STATE_UNKNOWN; 5]);
    let result = format!("{:?}", dfa);
    assert_eq!(result, "{}");
}

#[test]
fn test_fmt_with_dead_state() {
    let dfa = Dfa(vec![STATE_UNKNOWN, STATE_DEAD, STATE_UNKNOWN]);
    let result = format!("{:?}", dfa);
    assert_eq!(result, "{State 0: DEAD, State 1: DEAD}");
}

#[test]
fn test_fmt_with_non_dead_state() {
    let dfa = Dfa(vec![STATE_UNKNOWN, 2, STATE_DEAD, 3]);
    let result = format!("{:?}", dfa);
    assert_eq!(result, "{State 1: 2, State 2: DEAD, State 3: 3}");
}

#[test]
fn test_fmt_boundary_conditions() {
    let dfa = Dfa(vec![STATE_DEAD; 1e6 as usize]);
    let result = format!("{:?}", dfa);
    assert_eq!(result, "{State 0: DEAD}");
}

