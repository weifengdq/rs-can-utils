use colorful::{Color, Colorful};
use std::io::{BufRead, BufReader, Error};
use std::process::{Command, Stdio};
// use std::env;

fn main() -> Result<(), Error> {
    // loop through all args
    loop {
        // get pwd and print it
        let pwd = std::env::current_dir().unwrap();
        let pwd = pwd.to_str().unwrap();
        println!("\n{} >", pwd.gradient(Color::Blue));

        // read input new line
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // remove new line
        input.pop();
        // split input into args
        let mut args = input.split(" ").collect::<Vec<&str>>();
        // get app name
        let app = args.remove(0);

        let mut script = String::new();
        script.push_str(&app);
        for arg in args {
            script.push_str(" ");
            script.push_str(&arg);
        }

        let mut child = Command::new("sh")
            .arg("-c")
            .arg(&script)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        // if script equal to "exit" then break
        if script == "exit" {
            break;
        }

        // if script equal to "clear" then clear screen
        if script == "clear" {
            print!("{}[2J", 27 as char);
            println!("{}[1;1H", 27 as char);
            continue;
        }

        // if script contians "cd" then change dir
        if script.contains("cd") {
            let mut cd = script.split(" ").collect::<Vec<&str>>();
            cd.remove(0);
            let cd = cd.join(" ");
            std::env::set_current_dir(cd).unwrap();
            continue;
        }

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
            // let line = line.replace(&app, &app_name);
            println!("{}", line.gradient(Color::Red));
        }
    }

    Ok(())
}
