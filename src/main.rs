use std::io::{stdin, Read};
use arboard::Clipboard;

fn main() {
    let mut stdin = stdin();
    // let mut hundle = stdin.lock();
    let mut input = Vec::new();

    match stdin.read_to_end(&mut input) {
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
                eprintln!("utf8 transformation error: {e}");
                return;
            }
        },
        Err(e) => {
            eprintln!("clipboard set error: {e}");
            return;
        }
    };
}
