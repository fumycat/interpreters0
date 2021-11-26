use std::io;
use std::fs;
use std::env;

fn run(line: String) {
    println!("{}", line)
}

fn run_prompt() {
    let mut l = String::new();

    io::stdin()
        .read_line(&mut l)
        .expect("Failed to read line");
    
    run(l)
}

fn run_file(path: String) {
    let content = fs::read_to_string(path)
        .expect("Error while reading file");
    run(content)
}

fn main() {
    let args = env::args();
    let (n, _) = args.size_hint();
    match n {
        1 => run_prompt(),
        2 => run_file(args.last().unwrap()),
        _ => println!("Usage: ./exe [Filename]"),
    }

}
