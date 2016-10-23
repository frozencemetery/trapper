use std::io::*;
use std::process::{Command, Stdio};

mod tsm;

extern crate libc;
use std::char::from_u32;

/* I'm so unhappy about this */
fn shit_strerror() {
    unsafe {
        let e = *libc::__errno_location();
        let s = libc::strerror(e);
        let len = libc::strlen(s);
        let slice = std::slice::from_raw_parts(s as *mut u8, len);
        let s = std::str::from_utf8(slice).unwrap();
        println!("{}: {}", e, s);
    }
}

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
