use colorful::{Color, Colorful};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error};
// use std::io::ErrorKind;
use std::env;

fn main() -> Result<(), Error> {
    let app_name = env::args().nth(0).unwrap();
    let mut app_name = app_name.split("/").collect::<Vec<&str>>();
    let app_name = app_name.pop().unwrap();
    
    let app = "candump";
    let mut script = String::new();
    script.push_str(&app);
    for arg in env::args().skip(1) {
        script.push_str(" ");
        script.push_str(&arg);
    }

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(script)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let stdout = BufReader::new(stdout);
    let stderr = BufReader::new(stderr);

    for line in stdout.lines() {
        let line = line.unwrap();
        println!("{}", line.gradient(Color::Green));
    }

    for line in stderr.lines() {
        let line = line.unwrap();
        // replace cansend with app_name
        let line = line.replace(&app, &app_name);
        println!("{}", line.gradient(Color::Red));
    }

    Ok(())

    // let status = child.wait().unwrap();
    // if status.success() {
    //     Ok(())
    // } else {
    //     Err(Error::new(ErrorKind::Other, "failed to execute process"))
    // }
}
