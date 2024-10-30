use std::{fs, io::Read};
use bfrs::{scanner, interpreter};

const BFRS_VERSION: &str  = "1.0.0";


fn is_bf_cmd(char: &u8) -> bool {
    match *char {
        b'>' | b'<' | b'+' | b'-' | b'.' | b',' | b'[' | b']'  => true,
        _ => false,
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("bfrs v{BFRS_VERSION} - https://github.com/thehxdev/bfrs");
        eprintln!("Usage: bfrs <bf-source-file>");
        std::process::exit(1);
    }

    let source_file_path: &String = &args[1];
    let mut source_content: Vec<u8> = Vec::new();

    let mut source_file = fs::File::open(source_file_path)
        .expect("[ERROR] Failed to open the source file");

    source_file
        .read_to_end(&mut source_content)
        .expect("[ERROR] Failed to read the source file content");

    let cmds = source_content
        .into_iter()
        .filter(|char| is_bf_cmd(char))
        .collect();

    let cmds = scanner::tokenize_cmds(cmds);

    interpreter::execute(cmds)
}
