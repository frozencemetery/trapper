use std::io::*;
use std::process::{Command, Stdio};

mod tsm;

fn main() {
    let nh = Command::new("nethack")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute child")
        ;

    let mut stdout = nh.stdout.expect("No stdout...?");
    let mut stdin = nh.stdin.expect("No stdin...?");

    stdin.write(b"y  #quit\nyq");

    let mut res = vec![];
    stdout.read_to_end(&mut res);
    println!("{}", String::from_utf8(res).unwrap());
}
