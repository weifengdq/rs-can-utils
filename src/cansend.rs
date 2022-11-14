use colorful::{Color, Colorful};
use run_script::ScriptOptions;
use std::env;

fn main() {
    let app_name = env::args().nth(0).unwrap();
    let mut app_name = app_name.split("/").collect::<Vec<&str>>();
    let app_name = app_name.pop().unwrap();
    
    let app = "cansend";
    let mut script = String::new();
    script.push_str(&app);
    for arg in env::args().skip(1) {
        script.push_str(" ");
        script.push_str(&arg);
    }

    let options = ScriptOptions::new();
    let (code, stdout, stderr) = run_script::run(&script, &vec![], &options).unwrap();
    if code != 0 {
        for line in stderr.lines() {
            // replace cansend with app_name
            let line = line.replace(&app, &app_name);
            println!("{}", line.gradient(Color::Red));
        }
    } else {
        if stdout != "" {
            println!("{}", stdout);
        }
    }
}
