use std::env::args;
use std::fs::File;
use std::io;
// imported trait impls.
use std::io::BufRead;
use std::io::Write;

fn run_file(file: &str) {
    let mut f = File::open(file).expect(&format!("failed to open {}!", file));
    
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        print!("> ");
        // make sure we're not the victim of overly aggressive output buffering.
        io::stdout().flush().expect("failed to flush stdout!");
        if let Some(Ok(line)) = lines.next() {
            if !line.is_empty() { run(&line); }
        }
    }
}

fn run(code: &str) {
    unimplemented!();
}

fn main() {
    match args().len() {
        1 => run_prompt(),
        2 => run_file(&args().nth(1).unwrap()),
        _ => println!("usage: lox [script]")
    }
}
