use std::fs::File;
use std::io::{stdin, Read};
use std::env;
use arboard::Clipboard;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = Vec::new();

    let r_result = if args.len() >= 2 {
        let mut infile = match File::open(args[1].clone()) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("open {}: {e}", args[1]);
                return;
            }
        };
        infile.read_to_end(&mut input)
    } else {
        stdin().read_to_end(&mut input)
    };
    

    match r_result {
        Ok(_) => (),
        Err(e) => {
            eprintln!("read error: {e}");
            return;
        }
    };

    let mut clipboard = match Clipboard::new() {
        Ok(cb) => cb,
        Err(e) => {
            eprintln!("clipboard init error: {e}");
            return;
        }
    };

    match String::from_utf8(input) {
        Ok(s) => match clipboard.set_text(s) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("clipboard set error: {e}");
                return;
            }
        },
        Err(e) => {
            eprintln!("utf8 transformation error: {e}");
            return;
        }
    };
}
