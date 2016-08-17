use std::io::*;
use std::process::*;

fn main() {
    let mut nh = Command::new("ls")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute child");

    let mut res = vec![];

    let mut stdin = nh.stdin.unwrap();
    let mut stdout = nh.stdout.unwrap();

    stdout.read_to_end(&mut res);
    println!("{}", String::from_utf8(res).unwrap());
}
