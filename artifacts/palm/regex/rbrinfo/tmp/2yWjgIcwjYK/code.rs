fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Inst::*;

        fn with_goto(cur: usize, goto: usize, fmtd: String) -> String {
            if goto == cur + 1 {
                fmtd
            } else {
                format!("{} (goto: {})", fmtd, goto)
            }
        }

        fn visible_byte(b: u8) -> String {
            use std::ascii::escape_default;
            let escaped = escape_default(b).collect::<Vec<u8>>();
            String::from_utf8_lossy(&escaped).into_owned()
        }

        for (pc, inst) in self.iter().enumerate() {
            match *inst {
                Match(slot) => {
                    write!(f, "{:04} Match({:?})", pc, slot)?
                }
                Save(ref inst) => {
                    let s = format!("{:04} Save({})", pc, inst.slot);
                    write!(f, "{}", with_goto(pc, inst.goto, s))?;
                }
                Split(ref inst) => {
                    write!(
                        f, "{:04} Split({}, {})", pc, inst.goto1, inst.goto2)?;
                }
                EmptyLook(ref inst) => {
                    let s = format!("{:?}", inst.look);
                    write!(f, "{:04} {}", pc, with_goto(pc, inst.goto, s))?;
                }
                Char(ref inst) => {
                    let s = format!("{:?}", inst.c);
                    write!(f, "{:04} {}", pc, with_goto(pc, inst.goto, s))?;
                }
                Ranges(ref inst) => {
                    let ranges = inst.ranges
                        .iter()
                        .map(|r| format!("{:?}-{:?}", r.0, r.1))
                        .collect::<Vec<String>>()
                        .join(", ");
                    write!(
                        f, "{:04} {}", pc, with_goto(pc, inst.goto, ranges))?;
                }
                Bytes(ref inst) => {
                    let s = format!(
                        "Bytes({}, {})",
                        visible_byte(inst.start),
                        visible_byte(inst.end));
                    write!(f, "{:04} {}", pc, with_goto(pc, inst.goto, s))?;
                }
            }
            if pc == self.start {
                write!(f, " (start)")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }