use std::io::{stdin, stdout, Read, Write};
use crate::scanner::BFCmd;

const DEFAULT_CAP: usize = 1 << 16;

pub fn execute(tks: Vec<BFCmd>) {
    let mut arr: [u8; DEFAULT_CAP] = [0; DEFAULT_CAP];
    let mut ptr: usize = 0;
    let mut i: usize = 0;
    let mut lock = stdout().lock();

    while i < tks.len() {
        match &tks[i] {
            BFCmd::IncPtr(info) => ptr += info.rep,

            BFCmd::DecPtr(info) => ptr -= info.rep,

            BFCmd::IncVal(info) => arr[ptr] += (info.rep) as u8,

            BFCmd::DecVal(info) => arr[ptr] -= (info.rep) as u8,

            BFCmd::Write(info) => {
                let mut j = info.rep;
                while j > 0 {
                    write!(lock, "{}", arr[ptr] as char).unwrap();
                    j -= 1;
                }
            }

            BFCmd::Read(_) => {
                let mut buff: [u8; 1] = [0; 1];
                stdin()
                    .read(&mut buff)
                    .expect("Failed to read from stdin");
                arr[ptr] = buff[0];
            }

            BFCmd::JmpF(info) => { 
                if arr[ptr] == 0 {
                    i = info.m_idx
                }
            }

            BFCmd::JmpB(info) => {
                if arr[ptr] != 0 {
                    i = info.m_idx
                }
            }
        } // end match

        i += 1;
    } // end while loop
}
