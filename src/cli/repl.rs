use crate::cli::error::{Mode, run_once};
use std::io;
use std::io::Write;

pub fn run_repl(arg: Mode) {
    println!("simple calculator, :q to exit, :demo for demo, :h for help");
    let mut expr = String::new();
    loop {
        print!("calc REPL: ");
        io::stdout().flush().expect("flush failed");
        expr.clear();
        io::stdin()
            .read_line(&mut expr)
            .expect("Failed to read line");

        match expr.as_str().trim() {
            ":q" => break,
            ":demo" => {
                println!("calc REPL: 1 + 1");
                let tmp = run_once("1 + 1", arg.clone());
                match tmp {
                    Ok(()) => continue,
                    Err(e) => println!("Error: {}", e.render("1 + 1")),
                }
            }
            ":h" => {
                println!("simple calculator, :q to exit, :demo for demo, :h for help");
                println!("It is a calculator that can perfrom + - * /");
                println!("You can use it via import, REPL or using args");
            }
            expr => {
                if let Err(e) = run_once(expr, arg.clone()) {
                    println!("Error: {}", e.render(expr));
                }
            }
        };
    }
}
