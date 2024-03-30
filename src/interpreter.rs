use std::io::{stdin, Read};
use crate::scanner::BFCmd;

const DEFAULT_CAP: usize = 1 << 16;

pub fn execute(tks: Vec<BFCmd>) {
    let mut arr: [u8; DEFAULT_CAP] = [0; DEFAULT_CAP];
    let mut ptr: usize = 0;
    let mut i: usize = 0;
    let mut j: usize;

    while i < tks.len() {
        match &tks[i] {
            BFCmd::IncPtr(info) => ptr += info.rep,

            BFCmd::DecPtr(info) => ptr -= info.rep,

            BFCmd::IncVal(info) => arr[ptr] += (info.rep) as u8,

            BFCmd::DecVal(info) => arr[ptr] -= (info.rep) as u8,

            BFCmd::Write(info) => {
                j = 0;
                while j < info.rep {
                    print!("{}", arr[ptr] as char);
                    j += 1;
                }
            }

            BFCmd::Read(info) => {
                j = 0;
                while j < info.rep {
                    let mut buff: [u8; 1] = [0; 1];
                    stdin()
                        .read(&mut buff)
                        .expect("Failed to read from stdin");
                    arr[ptr] = buff[0];
                    j += 1;
                }
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
