use std::env;
use std::io;
use std::io::Write;

use wcal::{calculator, parser};

fn cmd_mod(cmd: &[String]) {
    let mut imod = true;
    for expr in cmd.iter() {
        match expr.as_str() {
            "-i" => imod = true,
            "-f" => imod = false,
            _ => {
                if imod {
                    println!("{}> {}", "i", expr);
                    match calculator!(expr, i128) {
                        Ok(res) => println!("{}", res),
                        Err(err) => println!("Error: {}", err)
                    }
                } else {
                    println!("{}> {}", "f", expr);
                    match calculator!(expr, f64) {
                        Ok(res) => println!("{}", res),
                        Err(err) => println!("Error: {}", err)
                    }
                }
            }
        }
    }
}

fn interactive_mod() {
    let mut imod = true;
    loop {
        if imod {
            print!("{}> ", "i");
        } else {
            print!("{}> ", "f");
        }
        io::stdout().flush().expect("Flush failed");
        let mut input = String::new();
        
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        
        match input {
            "i" | "i128" => {
                println!("Enter i128 mod");
                imod = true
            }
            "f" | "f64" => {
                println!("Enter f64 mod");
                imod = false
            }
            "q" | "quit" => {
                println!("Bye!");
                std::process::exit(0);
            }
            "h" | "help" => {
                println!("i\tEnter i128 mod");
                println!("f\tEnter f64 mod");
                println!("quit");
                println!("q\tQuit");
            }
            _ => {
                if imod {
                    match calculator!(input, i128) {
                        Ok(res) => println!("{}", res),
                        Err(err) => println!("Error: {}", err)
                    }
                } else {
                    match calculator!(input, f64) {
                        Ok(res) => println!("{}", res),
                        Err(err) => println!("Error: {}", err)
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        interactive_mod();
    } else {
        cmd_mod(&args[1..]);
    }
}
