#[derive(Debug)]
pub struct BFCmdInfo {
    pub rep: usize,
    pub m_idx: usize,
}

#[derive(Debug)]
pub enum BFCmd {
    IncPtr  (BFCmdInfo),
    DecPtr  (BFCmdInfo),
    IncVal  (BFCmdInfo),
    DecVal  (BFCmdInfo),
    Write   (BFCmdInfo),
    Read    (BFCmdInfo),
    JmpF    (BFCmdInfo),
    JmpB    (BFCmdInfo),
}


fn handle_reps(cmds: &Vec<u8>, idx: &mut usize) -> BFCmdInfo {
    let mut info = BFCmdInfo { rep: 1, m_idx: 0 };
    let ch = cmds[*idx];
    *idx += 1;

    if *idx == cmds.len() {
        return info;
    }

    while ch == cmds[*idx] {
        info.rep += 1;
        *idx += 1;
    }

    // decrement `i` by one because in tokenizer
    // it will be incremented again.
    *idx -= 1;
    return info;
}


fn handle_jump(tks: &mut Vec<BFCmd>) -> BFCmdInfo {
    let len = tks.len();
    let mut i = len - 1;
    let mut info = BFCmdInfo { rep: 1, m_idx: 0 };
    let mut nest = 0;

    while (i as isize) >= 0 {
        match &mut tks[i] {
            BFCmd::JmpB(_) => {
                nest += 1;
            }

            BFCmd::JmpF(m_info) => {
                if nest > 0 {
                    nest -= 1;
                } else {
                    m_info.m_idx = len;
                    info.m_idx = i;
                    return info;
                }
            }

            _ => {}
        }
        i -= 1;
    }

    return info;
}


pub fn tokenize_cmds(cmds: Vec<u8>) -> Vec<BFCmd> {
    let mut i = 0;
    let mut info: BFCmdInfo;
    let mut tks: Vec<BFCmd> = Vec::new();

    while i < cmds.len() {
        match cmds[i] {
            b'>' => {
                info = handle_reps(&cmds, &mut i);
                tks.push(BFCmd::IncPtr(info));
            }

            b'<' => {
                info = handle_reps(&cmds, &mut i);
                tks.push(BFCmd::DecPtr(info));
            }

            b'+' => {
                info = handle_reps(&cmds, &mut i);
                tks.push(BFCmd::IncVal(info));
            }

            b'-' => {
                info = handle_reps(&cmds, &mut i);
                tks.push(BFCmd::DecVal(info));
            }


            b'.' => {
                info = handle_reps(&cmds, &mut i);
                tks.push(BFCmd::Write(info));
            }


            b',' => {
                info = handle_reps(&cmds, &mut i);
                tks.push(BFCmd::Read(info));
            }

            b'[' => {
                tks.push(BFCmd::JmpF(BFCmdInfo { rep: 1, m_idx: 0 }));
            }

            b']' => {
                info = handle_jump(&mut tks);
                tks.push(BFCmd::JmpB(info));
            }

            _ => {},
        }
        i += 1;
    }

    return tks;
}
